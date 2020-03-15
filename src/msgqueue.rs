use std::any::{Any, TypeId};
use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};

#[derive(Clone)]
pub struct ServiceId {
    id: SrvId,
    message_queue: Weak<Mutex<MessageQueue>>,
}

impl ServiceId {
    fn new(id: SrvId, message_queue: &Arc<Mutex<MessageQueue>>) -> Self {
        let message_queue = Arc::downgrade(message_queue);
        Self { id, message_queue }
    }
    pub fn with_message_queue<T, F>(&self, func: F) -> Option<T>
    where
        F: FnOnce(&mut MessageQueue) -> Option<T>,
    {
        func(&mut *self.message_queue.upgrade()?.lock().ok()?)
    }
    pub fn put_state<T: Any + Send>(&self, data: T) {
        self.with_message_queue(|q| {
            q.put_state(&self.id, data);
            Some(())
        });
    }
    pub fn clone_state<T: Any + Clone>(&self) -> Option<T> {
        self.with_message_queue(|q| q.clone_state(&self.id))
    }
    pub fn peek_state<T: Any, V, F: FnOnce(&T) -> V>(&self, peek_func: F) -> Option<V> {
        self.with_message_queue(|q| q.peek_state(&self.id, peek_func))
    }
    pub fn poke_state<T: Any, V, F: FnOnce(&mut T) -> V>(&self, poke_func: F) -> Option<V> {
        self.with_message_queue(|q| q.poke_state(&self.id, poke_func))
    }
}

pub struct ServiceReg {
    id: SrvId,
    message_queue: Arc<Mutex<MessageQueue>>,
}

impl ServiceReg {
    pub fn new(message_queue: Arc<Mutex<MessageQueue>>) -> Self {
        let id = message_queue.lock().unwrap().register();
        Self { id, message_queue }
    }
    pub fn service_id(&self) -> ServiceId {
        ServiceId::new(self.id, &self.message_queue)
    }
}

impl Drop for ServiceReg {
    fn drop(&mut self) {
        self.message_queue.lock().unwrap().unregister(self.id)
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
struct SrvId(usize);

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
struct ReqId(usize);

impl ReqId {
    fn next(&self) -> Self {
        Self { 0: self.0 + 1 }
    }
}

type StateMap = HashMap<TypeId, Box<dyn Any + Send>>;
type RequestsQueue = VecDeque<(ReqId, Box<dyn Any + Send>)>;

#[derive(Debug)]
pub struct MessageQueue {
    services: HashMap<SrvId, (StateMap, RequestsQueue)>,
    responses: HashMap<ReqId, Option<Box<dyn Any + Send>>>,
    wakers: HashMap<ReqId, Waker>,
    next_srv_id: SrvId,
    next_req_id: ReqId,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            responses: HashMap::new(),
            wakers: HashMap::new(),
            next_srv_id: SrvId::default(),
            next_req_id: ReqId::default(),
        }
    }

    fn register(&mut self) -> SrvId {
        let id = self.next_srv_id;
        self.services
            .insert(id, (StateMap::new(), RequestsQueue::new()));
        self.next_srv_id = SrvId {
            0: self.next_srv_id.0 + 1,
        };
        id
    }
    fn unregister(&mut self, srv_id: SrvId) {
        self.services.remove(&srv_id);
    }

    fn state_map_mut(&mut self, srv_id: &SrvId) -> Option<&mut StateMap> {
        self.services
            .get_mut(srv_id)
            .map(|(state_map, _)| state_map)
    }

    fn state_map(&self, srv_id: &SrvId) -> Option<&StateMap> {
        self.services.get(srv_id).map(|(state_map, _)| state_map)
    }

    fn requests_queue_mut(&mut self, srv_id: &SrvId) -> Option<&mut RequestsQueue> {
        self.services.get_mut(srv_id).map(|(_, queue)| queue)
    }

    fn put_state<T: Any + Send>(&mut self, srv_id: &SrvId, data: T) {
        if let Some(state_map) = self.state_map_mut(srv_id) {
            state_map.insert(data.type_id(), Box::new(data));
        } else {
            let mut state_map = StateMap::new();
            state_map.insert(data.type_id(), Box::new(data));
            self.services
                .insert(srv_id.clone(), (state_map, RequestsQueue::new()));
        }
    }
    fn peek_state<T: Any, V, F: FnOnce(&T) -> V>(&self, srv_id: &SrvId, peek_func: F) -> Option<V> {
        self.state_map(srv_id)?
            .get(&TypeId::of::<T>())?
            .downcast_ref::<T>()
            .map(|v| peek_func(v))
    }
    fn poke_state<T: Any, V, F: FnOnce(&mut T) -> V>(
        &mut self,
        srv_id: &SrvId,
        poke_func: F,
    ) -> Option<V> {
        self.state_map_mut(srv_id)?
            .get_mut(&TypeId::of::<T>())?
            .downcast_mut::<T>()
            .map(|v| poke_func(v))
    }

    fn clone_state<T: Any + Clone>(&self, srv_id: &SrvId) -> Option<T> {
        self.state_map(srv_id)?
            .get(&TypeId::of::<T>())?
            .downcast_ref::<T>()
            .map(|v| v.clone())
    }
    fn remove_state<T: Any>(&mut self, srv_id: &SrvId) -> Option<T> {
        self.state_map_mut(srv_id)?
            .remove(&TypeId::of::<T>())
            .and_then(|v| v.downcast::<T>().ok().map(|v| *v))
    }

    fn post_request(&mut self, srv_id: &SrvId, request: Box<dyn Any + Send>) -> Option<ReqId> {
        let req_id = self.next_req_id;
        {
            let queue = self.requests_queue_mut(srv_id)?;
            queue.push_front((req_id, request));
        }
        self.next_req_id = self.next_req_id.next();
        Some(req_id)
    }
    fn take_request(&mut self, srv_id: &SrvId) -> Option<(ReqId, Box<dyn Any + Send>)> {
        self.requests_queue_mut(srv_id)?.pop_back()
    }
    fn set_response(&mut self, req_id: ReqId, resp: Option<Box<dyn Any + Send>>) {
        self.responses.insert(req_id, resp);
        if let Some(waker) = self.wakers.remove(&req_id) {
            waker.wake()
        }
    }
    fn check_response(
        &mut self,
        srv_id: SrvId,
        req_id: ReqId,
        waker: Waker,
    ) -> Result<Option<Box<dyn Any + Send>>, ()> {
        match self.responses.remove(&req_id) {
            // Normal response
            Some(resp @ Some(_)) => Ok(resp),
            // Request handled by service but not recognized
            Some(None) => Err(()),
            None => {
                if self.services.contains_key(&srv_id) {
                    // Response pending
                    self.wakers.insert(req_id, waker);
                    Ok(None)
                } else {
                    // No service to answer
                    Err(())
                }
            }
        }
    }
}

#[derive(Debug)]
struct Request {
    queue: Weak<Mutex<MessageQueue>>,
    srv_id: SrvId,
    opt_req_id: Option<ReqId>,
}

impl Request {
    fn new(queue: Arc<Mutex<MessageQueue>>, srv_id: SrvId, opt_req_id: Option<ReqId>) -> Self {
        Self {
            queue: Arc::downgrade(&queue),
            srv_id,
            opt_req_id,
        }
    }
}

impl Future for Request {
    type Output = Result<Box<dyn Any>, ()>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(req_id) = self.opt_req_id {
            if let Some(queue) = self.queue.upgrade() {
                match queue.try_lock().unwrap().check_response(
                    self.srv_id,
                    req_id,
                    cx.waker().clone(),
                ) {
                    Ok(Some(resp)) => Poll::Ready(Ok(resp)),
                    Ok(None) => Poll::Pending,
                    Err(_) => Poll::Ready(Err(())),
                }
            } else {
                Poll::Pending
            }
        } else {
            Poll::Ready(Err(()))
        }
    }
}
