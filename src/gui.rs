use crate::msgqueue::ServiceId;
use async_trait::async_trait;
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

pub struct CommonWidgetState {
    pub visible: bool,
    pub enabled: bool,
    pub label: String,
    pub rect: Rect,
}

impl CommonWidgetState {
    fn new() -> Self {
        Self {
            visible: true,
            enabled: true,
            label: "Default".into(),
            rect: Rect::zero(),
        }
    }
    fn peek<V, F: FnOnce(&Self) -> V>(id: ServiceId, peek_func: F) -> Option<V> {
        Some(id.peek_state(peek_func)?)
    }
    fn poke<V, F: FnOnce(&mut Self) -> V>(id: ServiceId, poke_func: F) -> Option<V> {
        Some(id.poke_state(poke_func)?)
    }
}

impl Default for CommonWidgetState {
    fn default() -> Self {
        Self::new()
    }
}

pub trait IService {
    fn service_id(&self) -> ServiceId;
}

#[async_trait]
pub trait IWidget: IService {
    fn get_label(&self) -> Option<String> {
        CommonWidgetState::peek(self.service_id(), |s| s.label.clone())
    }
    async fn set_label(&self, label: String) -> Option<()> {
        CommonWidgetState::poke(self.service_id(), |s| s.label = label)
    }
    fn get_enabled(&self) -> Option<bool> {
        CommonWidgetState::peek(self.service_id(), |s| s.enabled)
    }
    async fn set_enabled(&self, enabled: bool) -> Option<()> {
        CommonWidgetState::poke(self.service_id(), |s| s.enabled = enabled)
    }
    fn get_visible(&self) -> Option<bool> {
        CommonWidgetState::peek(self.service_id(), |s| s.visible)
    }
    async fn set_visible(&self, visible: bool) -> Option<()> {
        CommonWidgetState::poke(self.service_id(), |s| s.visible = visible)
    }
    fn get_rect(&self) -> Option<Rect> {
        CommonWidgetState::peek(self.service_id(), |s| s.rect.clone())
    }
    async fn set_rect(&self, rect: Rect) -> Option<()> {
        CommonWidgetState::poke(self.service_id(), |s| s.rect = rect)
    }
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
