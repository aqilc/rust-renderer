use std::collections::HashMap;
use glow::*;
use crate::graphics::api::GraphicsAPI;
use crate::graphics::api::Vec2;
//use image::io;

#[derive(Debug)]
#[repr(C)]
pub struct ShapeData {
	pub pos: Vec2<f32>,
	pub tex: Vec2<f32>,
	pub col: [f32; 4]
}
impl ShapeData {
	fn new(pos: Vec2<f32>) -> Self {

		// oh my god rust doesn't even have basic ternary operators bruh im about to kms
		ShapeData { pos, tex: Vec2 { x: if pos.x > 0.0 { 1.0 } else { 0.0 }, y: if pos.y > 0.0 {1.0} else {0.0} }, col: [1.0; 4] }
	}
}


pub struct GLContext<'a> {
	pub gl: glow::Context,
	pub va: Option<glow::VertexArray>,
	pub vb: Option<glow::Buffer>,
	pub ib: Option<glow::Buffer>,
	pub program: Option<glow::Program>,

	pub texloc: Option<&'a glow::UniformLocation>,

	// For images :D
	pub iva: Option<glow::VertexArray>,
	pub ivb: Option<glow::Buffer>,
	// pub iib: Option<glow::Buffer>,

	pub uniforms: HashMap<String, i32>,
	pub shapedata: Vec<ShapeData>,
	prev_shp_size: usize,
  pub indexdata: Vec<u32>,
  prev_ind_size: usize,
  pub curfill: [f32; 4],

	pub textures: Vec<glow::NativeTexture>,

	pub window_size: glutin::dpi::PhysicalSize<u32>,
}

pub enum DrawPrimiv<'a> {
	Text(&'a str),
	Shape, Image
}


#[derive(Debug)]
pub enum OpenGLType { Float, Integer, Char }
#[derive(Debug)]
pub struct LayoutType { typeenum: OpenGLType, count: i32 }
#[derive(Debug)]
pub struct Layout {
	types: Vec<LayoutType>,
	pub stride: i32
}


impl Layout {
	pub const fn new() -> Self {
		Layout { types: Vec::<LayoutType>::new(), stride: 0 }
	}
  pub fn addf(&mut self, count: i32) -> &mut Self {
		self.types.push(LayoutType { count, typeenum: OpenGLType::Float });
		self.stride += 4 * count;
		self
	}
  pub fn addi(&mut self, count: i32) -> &mut Self {
		self.types.push(LayoutType { count, typeenum: OpenGLType::Integer });
		self.stride += 4 * count;
		self
	}

  pub fn addc(&mut self, count: i32) -> &mut Self {
		self.types.push(LayoutType { count, typeenum: OpenGLType::Char });
		self.stride += 1 * count;
		self
	}

	pub unsafe fn apply(&mut self, gl: &glow::Context) {
		let mut offset: i32 = 0;
		for i in 0..self.types.len() {
			let index: u32 = i.try_into().unwrap();
			gl.enable_vertex_attrib_array(index);
			match &self.types[i] {
				l @ LayoutType { typeenum: OpenGLType::Float, .. } => {
					gl.vertex_attrib_pointer_f32(index, l.count, glow::FLOAT, false, self.stride, offset); offset += 4 * l.count; }
				l @ LayoutType { typeenum: OpenGLType::Integer, .. } => {
					gl.vertex_attrib_pointer_i32(index, l.count, glow::INT, self.stride, offset); offset += 4 * l.count; }
        l @ LayoutType { typeenum: OpenGLType::Char, .. } => {
          gl.vertex_attrib_pointer_i32(index, l.count, glow::UNSIGNED_BYTE, self.stride, offset); offset += 1 * l.count; }
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
			gl: glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _),
			va: None, vb: None, ib: None, program: None, iva: None, ivb: None, texloc: None,
			shapedata: Vec::<ShapeData>::new(),
			indexdata: Vec::<u32>::new(),
			uniforms: HashMap::<String, i32>::new(),
      curfill: [1.0, 0.0, 0.0, 1.0],
			prev_ind_size: 0, prev_shp_size: 0, textures: Vec::<glow::NativeTexture>::new(),
			window_size: window.window().inner_size() }
	}

	pub unsafe fn texture(&mut self, buf: Vec<u8>, width: i32, format: i32) -> u32 /*the id*/ {
		let texture = self.gl.create_texture().ok();
		self.gl.bind_texture(glow::TEXTURE_2D, texture);

		// Sets the default texture params, might add a way to change them later
		self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
		self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
		self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
		self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

		self.gl.tex_image_2d(glow::TEXTURE_2D, 0, format, width, buf.len() as i32 / width, 0, format.try_into().unwrap(),
			glow::UNSIGNED_BYTE, Some(core::slice::from_raw_parts(buf.as_ptr() as *const u8, buf.len() * core::mem::size_of::<u8>())));

		self.textures.push(texture.unwrap());
		(self.textures.len() - 1) as u32
	}

	pub unsafe fn set_texture(&mut self, tex: u32) {
		// why is rust so painnnnnnnnnnnnnn like wth is it even making me do :cry:
		if self.texloc.is_none() { self.texloc = (&self.gl.get_uniform_location(self.program.unwrap(), "u_tex")).as_ref(); }
		self.gl.uniform_1_u32(self.texloc, tex);
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

	pub fn convert_screencoords(&self, arr: Vec<Vec2<i32>>) -> Vec<Vec2<f32>> {
		let ret = Vec::<Vec2<f32>>::with_capacity(arr.len());
		let w = self.window_size.width as i32; let wf = w as f32;
		let h = self.window_size.height as i32; let hf = h as f32;
		for i in arr {
			ret.push(Vec2 { x: (i.x - w / 2) as f32 / wf, y: (i.y - h / 2) as f32 / wf });
		}
		ret
	}

	pub unsafe fn load_shaders(&self, file: &str) -> glow::Program {

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
		self.set_texture(0);

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

  unsafe fn load_image(&mut self, file: &str) -> Result<u32, image::ImageError> {
    let img_raw = image::io::Reader::open(file)?.decode()?; // stupid rust rules.. why tf do you need to drop values bruh just keep them around so i don't need random unnecessary variables and spend like 10 mins figuring out stupid compiler messages
    let img = img_raw.as_rgb8().unwrap(); // rust so stupid sometimes ughhhhhh
		Ok(self.texture(img.to_vec(), img.width() as i32, glow::RGB8 as i32))
	}

	unsafe fn image(&mut self, image: u32, x: i32, y: i32, w: i32, h: i32) {
		self.set_texture(image);
		if self.iva.is_none() { self.iva = self.gl.create_vertex_array().ok(); self.ivb = self.gl.create_buffer().ok(); }
		let data = self.convert_screencoords(vec![Vec2 { x, y }, Vec2 { x:w, y:h }]);
		let upload = vec![ShapeData::new(data, ShapeData {}, ShapeData {}, ShapeData {}, ShapeData {}, ShapeData {},]
		
		self.gl.buffer_sub_data_u8_slice(glow::ARRAY_BUFFER, 0, core::slice::from_raw_parts());
		self.gl.draw_elements(glow::TRIANGLES, 4, glow::UNSIGNED_INT, 0);
		self.set_texture(0);
	}

	unsafe fn load_font(&mut self) -> u8 {
		todo!();
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
