use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Mutex;

pub enum ObjError {
    ObjIdNotFound,
    TypeNotAssignedToObjId
}

pub type ObjResult<T = ()> = Result<T, ObjError>;

lazy_static! {
    static ref GLOBAL_OBJ_STORAGE: Mutex<ObjStorage> = Mutex::new(ObjStorage::new());
    static ref NEXT_OBJ_ID: Mutex<usize> = Mutex::new(0);
}
struct ObjStorage {
    storage: HashMap<ObjId, HashMap<TypeId, Box<dyn Any + Send>>>,
}

impl ObjStorage {
    fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    fn assign(&mut self, obj_id: ObjId, v: impl Any + Send) {
        self.storage
            .entry(obj_id)
            .or_insert(HashMap::new())
            .insert(v.type_id(), Box::new(v));
    }

    fn update<T: Any + Send>(&mut self, obj_id: ObjId, f: FnOnce(v: T))

    fn remove_obj(&mut self, obj_id: ObjId) {
        self.storage.remove(&obj_id);
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct ObjId(usize);

impl ObjId {
    pub fn new() -> Self {
        Self({
            let mut next_obj_id = NEXT_OBJ_ID.lock().unwrap();
            let obj_id = *next_obj_id;
            *next_obj_id += 1;
            obj_id
        })
    }

    pub fn assign(&self, v: impl Any + Send) {
        let mut storage = GLOBAL_OBJ_STORAGE.lock().unwrap();
        storage.assign(*self, v);
    }

    pub fn remove_obj(&mut self) {
        let mut storage = GLOBAL_OBJ_STORAGE.lock().unwrap();
        storage.remove_obj(*self);
    }
}
