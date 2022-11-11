use std::ops;

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

impl_op_ex!(+ |a: &Vec2<f32>, b: &Vec2<f32>| -> Vec2<f32> { Vec2::<f32> { x: a.x + b.x, y: a.y + b.y } });
impl_op_ex!(- |a: &Vec2<f32>, b: &Vec2<f32>| -> Vec2<f32> { Vec2::<f32> { x: a.x - b.x, y: a.y - b.y } });
impl_op_ex!(* |a: &Vec2<f32>, b: &Vec2<f32>| -> Vec2<f32> { Vec2::<f32> { x: a.x * b.x, y: a.y * b.y } });
impl_op_ex!(/ |a: &Vec2<f32>, b: &Vec2<f32>| -> Vec2<f32> { Vec2::<f32> { x: a.x / b.x, y: a.y / b.y } });

pub trait GraphicsAPI {
	unsafe fn setup(&mut self) -> &mut dyn GraphicsAPI;
	unsafe fn draw(&mut self);
	unsafe fn destroy(&mut self);
	unsafe fn rect(&mut self, x: f32, y: f32, w: f32, h: f32);
  unsafe fn load_image(&mut self, file: &str) -> Result<u32, image::ImageError>;
  unsafe fn image(&mut self, image: u32, x: i32, y: i32, w: i32, h: i32);
	unsafe fn load_font(&mut self) -> u32;
}

