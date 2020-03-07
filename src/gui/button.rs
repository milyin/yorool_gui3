use crate::msgqueue::{MessageQueue, ServiceId, ServiceReg};
use ggez::graphics::Rect;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ButtonState {
    pub touched: bool,
    pub label: String,
    pub rect: Rect,
}

impl ButtonState {
    fn new(label: String) -> Self {
        Self {
            touched: false,
            label,
            rect: Rect::default(),
        }
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::new("Default".into())
    }
}

#[derive(Clone)]
pub struct ButtonId(ServiceId);

impl ButtonId {
    pub fn get_label(&self) -> Option<String> {
        Some(self.0.peek_state(|s: &ButtonState| s.label.clone())?)
    }
    pub fn set_label(&self, label: String) {}
}

pub struct Button {
    reg: ServiceReg,
    id: ButtonId,
}

impl Button {
    pub fn new(message_queue: Arc<Mutex<MessageQueue>>) -> Self {
        let reg = ServiceReg::new(message_queue);
        let id = ButtonId(reg.service_id());
        Self { reg, id }
    }
    pub fn button_id(&self) -> ButtonId {
        self.id.clone()
    }
}
