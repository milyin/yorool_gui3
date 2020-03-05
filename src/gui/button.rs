use crate::msgqueue::{MessageQueue, ServiceId};
use ggez::graphics::Rect;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum ButtonMode {
    PressButton,
    Checkbox(bool),
    Radio(bool),
}

#[derive(Clone)]
pub struct ButtonState {
    pub mode: ButtonMode,
    pub touched: bool,
    pub label: String,
    pub rect: Rect,
}

impl ButtonState {
    fn new(mode: ButtonMode, label: String) -> Self {
        Self {
            mode,
            touched: false,
            label,
            rect: Rect::default(),
        }
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::new(ButtonMode::PressButton, "Default".into())
    }
}

#[derive(Clone)]
pub struct ButtonId(ServiceId);

impl ButtonId {
    pub fn new(queue: Arc<Mutex<MessageQueue>>) -> Self {
        let id = ServiceId::new(queue);
        id.put_state(ButtonState::default());
        Self { 0: id }
    }
    pub fn get_mode(&self) -> Option<ButtonMode> {
        Some(self.0.clone_state::<ButtonState>()?.mode)
    }
}
