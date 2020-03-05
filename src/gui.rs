use ggez::event::MouseButton;
use ggez::graphics::Rect;
use ggez::{Context, GameResult};

pub mod button;

pub trait EventHandlerProxy {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }
}

pub struct WidgetState {
    pub visible: bool,
    pub enabled: bool,
    pub label: String,
    pub rect: Rect,
}
/*
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct WidgetId(ObjId);

impl WidgetId {
    pub fn new() -> Self {
        let id = ObjId::new();
        id.assign(WidgetState::new());
        Self(id)
    }
    pub fn set_visible(v: bool) {}
}

pub struct WidgetState {
    pub visible: bool,
    pub enabled: bool,
    pub label: String,
    pub rect: Rect,
}

impl WidgetState {
    fn new() -> Self {
        Self {
            visible: true,
            enabled: true,
            label: String::default(),
            rect: Rect::default(),
        }
    }
}

impl Default for WidgetState {
    fn default() -> Self {
        Self::new()
    }
}
*/
