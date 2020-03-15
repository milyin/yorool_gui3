use crate::gui::{CommonWidgetState, EventHandlerProxy, IService, IWidget};
use crate::msgqueue::{MessageQueue, ServiceId, ServiceReg};
use async_std::task;
use async_trait::async_trait;
use ggez::event::MouseButton;
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ButtonState {
    pub touched: bool,
}

impl ButtonState {
    fn new() -> Self {
        Self { touched: false }
    }
    fn peek<V, F: FnOnce(&Self) -> V>(id: ServiceId, peek_func: F) -> Option<V> {
        Some(id.peek_state(peek_func)?)
    }
    fn poke<V, F: FnOnce(&mut Self) -> V>(id: ServiceId, poke_func: F) -> Option<V> {
        Some(id.poke_state(poke_func)?)
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::new()
    }
}

pub trait ButtonSkin: Default {
    fn set_state(&mut self, state: &ButtonState);
    fn is_hot_area(&self, x: f32, y: f32) -> bool;
    fn draw(&mut self, _ctx: &mut Context) -> GameResult;
}

#[async_trait]
pub trait IButton: IService {
    fn get_touched(&self) -> Option<bool> {
        ButtonState::peek(self.service_id(), |s| s.touched)
    }
    async fn set_touched(&self, touched: bool) -> Option<()> {
        ButtonState::poke(self.service_id(), |s| s.touched = touched)
    }
}

#[derive(Clone)]
pub struct ButtonId(ServiceId);

impl ButtonId {}

impl IService for ButtonId {
    fn service_id(&self) -> ServiceId {
        self.0.clone()
    }
}

impl IButton for ButtonId {}
impl IWidget for ButtonId {}

pub struct Button<S: ButtonSkin> {
    reg: ServiceReg,
    id: ButtonId,
    skin: S,
}

impl<S: ButtonSkin> Button<S> {
    pub fn new(message_queue: Arc<Mutex<MessageQueue>>) -> Self {
        let reg = ServiceReg::new(message_queue);
        let srv_id = reg.service_id();
        srv_id.put_state(CommonWidgetState::default());
        srv_id.put_state(ButtonState::default());
        let id = ButtonId(srv_id);
        let skin = S::default();
        Self { reg, id, skin }
    }
    pub fn button_id(&self) -> ButtonId {
        self.id.clone()
    }
}

impl<S: ButtonSkin> EventHandlerProxy for Button<S> {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.skin.draw(ctx)
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        if _button == MouseButton::Left {
            if self.skin.is_hot_area(x, y) {
                let button_id = self.button_id();
                task::spawn(async move { button_id.set_touched(true).await });
            }
        }
    }
}
