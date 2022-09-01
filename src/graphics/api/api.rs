
// ------- Vector Datatypes -------
#[derive(Default, Copy, Clone)]
pub struct Vec2<T> {
	pub x: T,
	pub y: T,
}
impl<T> Vec2<T> {
	pub fn new(x: T, y: T) -> Vec2<T> { Vec2::<T> { x, y } }
	pub fn set(&mut self, x: T, y: T) { self.x = x; self.y = y; }
}

pub trait GraphicsAPI {
	unsafe fn setup(&mut self) -> &mut dyn GraphicsAPI;
	unsafe fn draw(&mut self);
	unsafe fn destroy(&mut self);
}
