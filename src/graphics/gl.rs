use std::collections::HashMap;
use glow::*;
use crate::graphics::api::GraphicsAPI;
use crate::graphics::api::Vec2;

#[derive(Debug)]
#[repr(C)]
pub struct ShapeData {
	pub pos: Vec2<f32>,
	pub tex: Vec2<f32>,
	pub col: [f32; 4]
}

pub struct GLContext {
	pub gl: glow::Context,
	pub va: Option<glow::VertexArray>,
	pub vb: Option<glow::Buffer>,
	pub ib: Option<glow::Buffer>,
	pub program: Option<glow::Program>,

	pub uniforms: HashMap<String, i32>,
	pub shapedata: Vec<ShapeData>,
	prev_shp_size: usize,
	pub indexdata: Vec<u32>,
    pub curfill: [f32; 4],
	prev_ind_size: usize,
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
			let index: u32 = i.try_into().unwrap();
			gl.enable_vertex_attrib_array(index);
			match &self.types[i] {
				l @ LayoutType { typeenum: OpenGLType::Float, .. } => {
					gl.vertex_attrib_pointer_f32(index, l.count as i32, glow::FLOAT, false, self.stride, offset); offset += 4 * l.count as i32; },
				l @ LayoutType { typeenum: OpenGLType::Integer, .. } => {
					gl.vertex_attrib_pointer_i32(index, l.count as i32, glow::INT, self.stride, offset); offset += 4 * l.count as i32; }
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
			gl: glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _), va: None, vb: None, ib: None, program: None,
			shapedata: Vec::<ShapeData>::new(),
			indexdata: Vec::<u32>::new(),
			uniforms: HashMap::<String, i32>::new(),
            curfill: [1.0, 0.0, 0.0, 1.0],
			prev_ind_size: 0, prev_shp_size: 0 }
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

		// Debug and other basic stuffs
		self.gl.debug_message_callback(|_: u32, _: u32, _: u32, _: u32, msg: &str| println!("{}", msg));
		self.gl.enable(glow::BLEND);
		self.gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_COLOR);

		// Creates a vertex array and loads shaders
		self.va = Some(self.gl.create_vertex_array().expect("bruh why won't VA form"));

		// Binds the vertex array so we can put the layout on it
		self.gl.bind_vertex_array(self.va);

		// Compiles shaders
		if cfg!(debug_assertions) {
			// Reads file dynamically if in debug mode, so we don't have to recompile when editing shaders
			self.program = Some(self.load_shaders(std::fs::read_to_string(r".\res\shaders.glsl").unwrap().as_str()))
		} else {
			self.program = Some(self.load_shaders(include_str!("../../res/shaders.glsl")));
		}

		// I FORGOT THIS INITIALLY LOL WTF
		self.gl.use_program(self.program);

		self.vb = self.gl.create_buffer().ok();
		self.ib = self.gl.create_buffer().ok();
		self.gl.bind_buffer(glow::ARRAY_BUFFER, self.vb);
		self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.ib);

		// Makes a new layout, and then adds it thru gl attrib array ptrs
		Layout::new().addf(2).addf(2).addf(4).apply(&self.gl); // (apply comes last because we need the stride)
		self
	}

	unsafe fn draw(&mut self) {
		if self.shapedata.len() == 0 { return; }

		// Vertex data upload
		self.gl.bind_buffer(glow::ARRAY_BUFFER, self.vb);

		// If vertex data length used to be under the size required for the data, make it bigger
		if self.prev_shp_size < self.shapedata.len() {
			self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, core::slice::from_raw_parts(self.shapedata.as_ptr() as *const u8,
				self.shapedata.len() * core::mem::size_of::<ShapeData>()), glow::STATIC_DRAW);
			self.prev_shp_size = self.shapedata.len();
		} else if self.shapedata.len() > 0 {
			self.gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, core::slice::from_raw_parts(self.shapedata.as_ptr() as *const u8,
				self.shapedata.len() * core::mem::size_of::<ShapeData>()));
		}

		// Index buffer upload
		self.gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.ib);
		if self.prev_ind_size < self.indexdata.len() {
			self.gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, core::slice::from_raw_parts(self.indexdata.as_ptr() as *const u8,
				self.indexdata.len() * core::mem::size_of::<u32>()), glow::STATIC_DRAW);
			self.prev_ind_size = self.indexdata.len();
		} else if self.indexdata.len() > 0 {
			self.gl.buffer_sub_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, 0, core::slice::from_raw_parts(self.indexdata.as_ptr() as *const u8,
				self.indexdata.len() * core::mem::size_of::<u32>()));
		}

		self.gl.clear(glow::COLOR_BUFFER_BIT);
		self.gl.draw_elements(glow::TRIANGLES, self.indexdata.len() as i32, glow::UNSIGNED_INT, 0);
		// for i in 0..self.shapedata.len() { println!("{}: {:?}", i, &self.shapedata[i]); }
		self.shapedata.clear();
		self.indexdata.clear();
	}

	unsafe fn destroy(&mut self) {
		self.gl.delete_buffer(self.vb.unwrap());
		self.gl.delete_buffer(self.ib.unwrap());
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
// 	use glium;

// 	unsafe {
// 		let event_loop = EventLoop::new();
// 		let window = glutin::ContextBuilder::new().with_multisampling(8);

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
