use std::collections::HashMap;
use glow::*;
use crate::graphics::api::api::GraphicsAPI;

pub struct GLContext {
	pub gl: glow::Context,
	pub va: Option<glow::VertexArray>,
	pub program: Option<glow::Program>,

	pub uniforms: HashMap<String, i32>
}


pub enum OpenGLType { Float, Integer, Char }
pub struct LayoutType { typeenum: OpenGLType, count: u8 }
pub struct Layout {
	types: Vec<LayoutType>,
	pub stride: i32
}
impl Layout {
	fn new() -> Self {
		Layout { types: Vec::<LayoutType>::new(), stride: 0 }
	}
	fn addf(&mut self, count: u8) -> &mut Self {
		self.types.push(LayoutType { count, typeenum: OpenGLType::Float });
		self.stride += 4;
		self
	}
	fn addi(&mut self, count: u8) -> &mut Self {
		self.types.push(LayoutType { count, typeenum: OpenGLType::Integer });
		self.stride += 4;
		self
	}

	unsafe fn apply(&mut self, gl: &glow::Context) {
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

impl GLContext {
	pub unsafe fn new(window: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) -> Self {
		GLContext { gl: glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _), va: None, program: None, uniforms: HashMap::<String, i32>::new() }
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
	unsafe fn setup(&mut self) {

		// Creates a vertex array and loads shaders
		self.va = Some(self.gl.create_vertex_array().expect("bruh why won't VA form"));
		self.program = Some(self.load_shaders(include_str!("../../../res/shaders.glsl")));

		// Binds the vertex array so we can put the layout on it
		self.gl.bind_vertex_array(self.va);

		// Makes a new layout, and then adds it thru gl attrib array ptrs
		Layout::new().addf(2).addf(2).addf(4).apply(&self.gl);
	}

	unsafe fn draw(&mut self) {
		self.gl.clear(glow::COLOR_BUFFER_BIT);
		self.gl.draw_arrays(glow::TRIANGLES, 0, 3);
	}

	unsafe fn destroy(&mut self) {
		self.gl.delete_program(self.program.unwrap());
		self.gl.delete_vertex_array(self.va.unwrap());
	}
}

