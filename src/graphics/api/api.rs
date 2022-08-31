
pub trait GraphicsAPI {
	unsafe fn setup(&mut self) -> &mut dyn GraphicsAPI;
	unsafe fn draw(&mut self);
	unsafe fn destroy(&mut self);
}
