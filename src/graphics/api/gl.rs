use std::collections::HashMap;
use glow::*;
use crate::graphics::api::api::GraphicsAPI;
use crate::graphics::api::api::Vec2;

#[derive(Debug)]
pub struct ShapeData {
	pub pos: Vec2<f32>,
	pub tex: Vec2<f32>,
	pub col: [f32; 4]
	
}
pub struct GLContext {
	pub gl: glow::Context,
	pub va: Option<glow::VertexArray>,
	pub vb: Option<glow::Buffer>,
	pub eb: Option<glow::Buffer>,
	pub program: Option<glow::Program>,

	pub uniforms: HashMap<String, i32>,
	pub shapedata: Vec<ShapeData>,
	pub indexdata: Vec<u32>,
}


#[derive(Debug)]
pub enum OpenGLType { Float, Integer, Char }
#[derive(Debug)]
pub struct LayoutType { typeenum: OpenGLType, count: u8 }
#[derive(Debug)]
pub struct Layout {
	types: Vec<LayoutType>,
	pub stride: i32
}


impl Layout {
	pub const fn new() -> Self {
		Layout { types: Vec::<LayoutType>::new(), stride: 0 }
	}
	pub fn addf(&mut self, count: u8) -> &mut Self {
		self.types.push(LayoutType { count, typeenum: OpenGLType::Float });
		self.stride += 4 * count as i32;
		self
	}
	pub fn addi(&mut self, count: u8) -> &mut Self {
		self.types.push(LayoutType { count, typeenum: OpenGLType::Integer });
		self.stride += 4 * count as i32;
		self
	}

	pub unsafe fn apply(&mut self, gl: &glow::Context) {
		let mut offset: i32 = 0;
		for i in 0..self.types.len() {
			gl.enable_vertex_attrib_array(i.try_into().unwrap());
			match &self.types[i] {
				l @ LayoutType { typeenum: OpenGLType::Float, .. } => { gl.vertex_attrib_pointer_f32(i.try_into().unwrap(), l.count as i32, glow::FLOAT, false, self.stride, offset); offset += 4 * l.count as i32; },
				l @ LayoutType { typeenum: OpenGLType::Integer, .. } => { gl.vertex_attrib_pointer_i32(i.try_into().unwrap(), l.count as i32, glow::INT, self.stride, offset); offset += 4 * l.count as i32; }
				_ => { return; }
			}
		}
	}
}

const TEXTUREH: f32 = 512.0;
const TEXTUREW: f32 = 512.0;
const TEXCOORDS: [Vec2<f32>; 4] = [Vec2::<f32>::new(1.0 - 2.5 / TEXTUREW, 1.0 - 2.5 / TEXTUREH),
	Vec2::<f32>::new(1.0 - 2.5 / TEXTUREW, 1.0), Vec2::<f32>::new(1.0, 1.0 - 2.5 / TEXTUREH), Vec2::<f32>::new(1.0, 1.0)];
impl GLContext {
	pub unsafe fn new(window: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) -> Self {
		GLContext {
			gl: glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _), va: None, vb: None, eb: None, program: None,
			shapedata: Vec::<ShapeData>::new(),
			indexdata: Vec::<u32>::new(),
			uniforms: HashMap::<String, i32>::new() }
	}

	pub fn push_shape(&mut self, points: Vec<Vec2<f32>>, index: Vec<u32>, color: [f32; 4]) -> &mut Self {

		// Stores length of shapedata so we can add it to each of the indexes later
		let len = self.shapedata.len();
		
		// Adds every point into the shapedata buffer
		for i in 0..points.len() {
			self.shapedata.push(ShapeData {
				col: color,
				pos: points[i],
				tex: TEXCOORDS[i % 4]
			});
		}

		// Adds every index to the whole index buffer, and since we're appending the shapes, we're adding the length of the shape buffer so the indexes are referencing the proper shapes
		for i in 0..index.len() {
			self.indexdata.push(len as u32 + index[i]); }
		
		self
	}
	
	pub unsafe fn load_shaders(self: &Self, file: &str) -> glow::Program {

		// Splits the file up by the string "# frag"
		let (vert, frag): (&str, &str) = {
			let mut split = file.split("# frag");
			(split.next().unwrap(), split.next().unwrap())
		};

		// Creates a new program so we can return
		let program = self.gl.create_program().expect("bruh i can't even create a program wtf is this");
		
		// Compiles shaders
		let compile = |t: u32, s: &str| -> glow::Shader {
			let shader = self.gl.create_shader(t).expect("bruh can't even create shaders bad");
			self.gl.shader_source(shader, s);
			self.gl.compile_shader(shader);
			if !self.gl.get_shader_compile_status(shader) {
				panic!("{}", self.gl.get_shader_info_log(shader));
			}
			self.gl.attach_shader(program, shader);

			shader
		};

		// Compiles vertex and fragment shaders
		let vs = compile(glow::VERTEX_SHADER, vert);
		let fs = compile(glow::FRAGMENT_SHADER, frag);

		// Links symbols with error checking
		self.gl.link_program(program);
		if !self.gl.get_program_link_status(program) {
			panic!("Error in linking programs: {}", self.gl.get_program_info_log(program));
		}

		// Deletes shaders and exits
		self.gl.delete_shader(vs);
		self.gl.delete_shader(fs);

		program
	}
}

impl GraphicsAPI for GLContext {
	unsafe fn setup(&mut self) -> &mut dyn GraphicsAPI {

		// Creates a vertex array and loads shaders
		self.va = Some(self.gl.create_vertex_array().expect("bruh why won't VA form"));

		// Binds the vertex array so we can put the layout on it
		self.gl.bind_vertex_array(self.va);

		// Makes a new layout, and then adds it thru gl attrib array ptrs
		Layout::new().addf(2).addf(2).addf(4).apply(&self.gl); // (apply comes last because we need the stride)

		// Compiles shaders
		self.program = Some(self.load_shaders(include_str!("../../../res/shaders.glsl")));

		self.eb = Some(self.gl.create_buffer().unwrap());
		self.vb = Some(self.gl.create_buffer().unwrap());
		self.gl.bind_buffer(glow::ARRAY_BUFFER, self.eb);
		self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.vb);
		self.gl.debug_message_callback(|_: u32, _: u32, _: u32, _: u32, msg: &str| println!("{}", msg));
		self.gl.enable(glow::BLEND);
		self.gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_COLOR);

		self
	}

	unsafe fn draw(&mut self) {
		self.gl.clear(glow::COLOR_BUFFER_BIT);
		self.gl.bind_buffer(glow::ARRAY_BUFFER, self.eb);
		self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.vb);
		self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, core::slice::from_raw_parts(self.shapedata.as_ptr() as *const u8,
			self.shapedata.len() * core::mem::size_of::<ShapeData>()), glow::DYNAMIC_DRAW);
		self.gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, core::slice::from_raw_parts(self.indexdata.as_ptr() as *const u8,
			self.indexdata.len() * core::mem::size_of::<u32>()), glow::DYNAMIC_DRAW);
		self.gl.draw_elements(glow::TRIANGLES, self.indexdata.len() as i32, glow::UNSIGNED_INT, 0);
		self.shapedata.clear();
		self.indexdata.clear();
	}

	unsafe fn destroy(&mut self) {
		self.gl.delete_vertex_array(self.va.unwrap());
		self.gl.delete_program(self.program.unwrap());
	}

	unsafe fn rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
		self.push_shape(Vec::<Vec2<f32>>::from([
			Vec2::<f32> { x, y },
			Vec2::<f32> { x: x + w, y },
			Vec2::<f32> { x, y: y + h },
			Vec2::<f32> { x: x + w, y: y + h },
		]), vec![0, 1, 2, 2, 1, 3], [1.0, 0.0, 0.0, 1.0]);
	}
}

// #[test]
// fn test_push() {
// 	use glutin::event_loop::EventLoop;

// 	unsafe {
// 		let event_loop: EventLoop<()> = EventLoop::any_thread();
// 		let window = glutin::ContextBuilder::new()
// 			.build_windowed(
// 				glutin::window::WindowBuilder::new().with_title("tetris").with_inner_size(glutin::dpi::LogicalSize::new(600., 400.)), &event_loop
// 			).unwrap().make_current().unwrap();

// 		// Sets everything up
// 		let mut gl = GLContext::new(&window);

// 		// Creates a vertex array and loads shaders
// 		gl.va = Some(gl.gl.create_vertex_array().expect("bruh why won't VA form"));
// 		gl.program = Some(gl.load_shaders(include_str!("../../../res/shaders.glsl")));

// 		// Binds the vertex array so we can put the layout on it
// 		gl.gl.bind_vertex_array(gl.va);

// 		// Makes a new layout, and then adds it thru gl attrib array ptrs
// 		let mut layout = Layout::new();
// 		layout.addf(2).addf(2).addf(4);
// 		print!("{:?}", layout);
// 		layout.apply(&gl.gl); // (apply comes last because we need the stride)

// 		gl.gl.bind_buffer(glow::ARRAY_BUFFER, Some(gl.gl.create_buffer().unwrap()));
// 		gl.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(gl.gl.create_buffer().unwrap()));

// 		gl.rect(0.0, 0.0, 0.5, 0.5);
// 		for i in gl.shapedata.iter() { print!("{:?}", i); }
// 	}
// }
