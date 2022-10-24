
// ------- Vector Datatypes -------
#[derive(Default, Copy, Clone, Debug)]
#[repr(C)]
pub struct Vec2<T> {
	pub x: T,
	pub y: T,
}
impl<T> Vec2<T> {
	pub const fn new(x: T, y: T) -> Vec2<T> { Vec2::<T> { x, y } }
	pub fn set(&mut self, x: T, y: T) { self.x = x; self.y = y; }
}

pub trait GraphicsAPI {
	unsafe fn setup(&mut self) -> &mut dyn GraphicsAPI;
	unsafe fn draw(&mut self);
	unsafe fn destroy(&mut self);
	unsafe fn rect(&mut self, x: f32, y: f32, w: f32, h: f32);
}