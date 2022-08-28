
pub trait GraphicsAPI {
	unsafe fn setup(&mut self);
	unsafe fn draw(&mut self);
	unsafe fn destroy(&mut self);
}
