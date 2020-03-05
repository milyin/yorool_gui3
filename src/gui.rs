use ggez::{Context, GameResult};
use ggez::event::MouseButton;

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
