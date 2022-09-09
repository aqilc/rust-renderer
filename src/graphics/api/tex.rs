use fontdue::*;
use std::collections::HashMap;
use std::fs::*;
use crate::graphics::api::api::Vec2;

// ----- Texture datatypes ------
#[derive(Copy, Clone)]
pub enum Channels { RGB = 3, RGBA = 4, GRAYSCALE = 1 }
pub struct Tex {
	pub w: usize,
	pub h: usize,
	pub data: Vec<u8>,
	pub channels: Channels
}
impl Tex {
	pub fn new(w: usize, h: usize, channels: Channels) -> Self {
		Tex { w, h, data: vec![0 as u8; w * h * channels as usize], channels }
	}
	pub fn resize(&mut self, w: usize, h: usize) -> &mut Self {
		let from = &self.data;
		let mut to = vec![0 as u8; w * h * self.channels as usize];
		for i in 0..self.h {
			for j in 0..self.w {
				to[i * self.w + j] = from[i * self.w + j];
			}
		}
		self.w = w; self.h = h; self.data = to;
		
		self
	}
	pub fn draw(&mut self, data: Vec<u8>, pos: Vec2<usize>, height: usize) {
		for y in 0..height {
			for x in 0..(data.len() / height) {
				self.data[(y + pos.y) * height + (pos.x + x)] = data[y * height + x];
			}
		}
	}
}

pub struct GlyphAttributes {
	pos: Vec2<u16>, size: Vec2<u16>, advance_x: u32
}
pub struct FontAtlas {
	pub tex: Tex,
	pub fonts: HashMap<String, Box<Font>>,
	lookup: HashMap<String, Box<GlyphAttributes>>,
	places: Node,
}
impl<'a> FontAtlas {
	const DEFAULTCHARS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890[]{}()/\\=+\'\"<>,.-_?|!@#$%^&* :";
	const STARTINGSIZE: &'static Vec2<u32> = &Vec2::<u32> { x: 128, y: 128 };
	pub fn new() -> Self {
		FontAtlas { fonts: HashMap::<String, Box<Font>>::new(),
			lookup: HashMap::<String, Box<GlyphAttributes>>::new(),
			places: Node::new(Vec2::<u32>::default(), *FontAtlas::STARTINGSIZE),
			tex: Tex::new(FontAtlas::STARTINGSIZE.x as usize, FontAtlas::STARTINGSIZE.y as usize, Channels::GRAYSCALE) }
	}

	// Loads all of the default chars for a font.
	pub fn load(&mut self, name: &str, path: &str) {
		let file: Vec<u8> = read(path).unwrap();
		self.fonts.insert(String::from(name), Box::new(Font::from_bytes(file, FontSettings::default()).unwrap()));
		
		for i in FontAtlas::DEFAULTCHARS.chars() {
			self.loadchar(i, name);
		}
	}
	pub fn loadchar(&mut self, character: char, font: &str) -> Result<(), String> {
		let (metrics, bitmap) = self.fonts[font].rasterize(character, 48.0);
		if let Some(pos) = self.places.pack(&Vec2::<u32> { x: metrics.width as u32, y: metrics.height as u32 }) {

			// Inserts the bitmap into the texture at the specified coords
			self.tex.draw(bitmap, Vec2::<usize> { x: pos.pos.x as usize, y: pos.pos.y as usize }, pos.size.y as usize);

			// Inserts the character into the hashmap so we can look it up later
			self.lookup.insert(String::from(character), Box::<GlyphAttributes>::new(GlyphAttributes {
				size: Vec2::<u16> { x: metrics.width as u16, y: metrics.height as u16 },
				pos: Vec2::<u16> { x: pos.pos.x as u16, y: pos.pos.y as u16 }, advance_x: (metrics.advance_width / 64.0) as u32
			}));
		} else { return Err(format!("Cannot insert character {} into this map because it doesn't fit.", character)); }
	}
}


// -------- Texture Packer -------
struct Node {
	pos: Vec2<u32>,
	size: Vec2<u32>,
	left: Option<Box<Node>>,
	right: Option<Box<Node>>,
	filled: bool,
}
impl Node {
	pub fn new(pos: Vec2<u32>, size: Vec2<u32>) -> Node {
		Node { pos, size, left: None, right: None, filled: false }
	}
	pub fn pack(&mut self, size: &Vec2<u32>) -> Option<&mut Node> {
		if self.left.is_none() || self.right.is_none() {

			// If we're filled or can't fit the thing
			if self.filled { return None; }
			if self.size.x < size.x || self.size.y < size.y { return None; }

			// If we're the exact match
			if self.size.x == size.x && self.size.y == size.y { self.filled = true; return Some(self); }

			
			// First node is always going to be positioned at the parent node
			let c1p = self.pos;

			// Vectors defining the metrics for the later children
			let [mut c2p, mut c1s, mut c2s] = [Vec2::<u32>::default(); 3];

			// The space between the edges of the box being inserted and the box outside
			let (dw, dh) = (self.size.x - size.x, self.size.y - size.y);

			if dw > dh {
				// Vertical Split
				c1s.x = size.x;
				c1s.y = self.size.y;

				c2p.x = self.pos.x + size.x;
				c2p.y = self.pos.y;
				c2s.x = self.size.x - size.x;
				c2s.y = self.size.y;
			} else {
				// Horizontal split
				c1s.x = self.size.x;
				c1s.y = size.y;

				c2p.x = self.pos.x;
				c2p.y = self.pos.y + size.y;
				c2s.x = self.size.x;
				c2s.y = self.size.y - size.y;
			}

			self.left = Some(Box::<Node>::new(Node::new(c1p, c1s)));
			self.right = Some(Box::<Node>::new(Node::new(c2p, c2s)));

			// Pack the node into the first child
			return self.left.as_mut().unwrap().pack(size);
		}

		let c1 = self.left.as_mut().unwrap().pack(size);
		if c1.is_none() { return self.right.as_mut().unwrap().pack(size); }
		return c1;
	}
}

#[test]
fn texture_pack() {
	use rand::Rng;
	let mut root = Node::new(Vec2::default(), Vec2::<u32> { x: 100, y: 100 });
	let mut nodes = vec![Vec2::<u32>::new(20, 30)];
	let mut cur = 1;
	
	assert!(root.pack(&nodes[0]).is_some());

	for _ in 0..30 {
		nodes.push(Vec2::<u32>::new(rand::thread_rng().gen_range(10..40), rand::thread_rng().gen_range(10..40)));
	}

	while cur < nodes.len() {
		if cur < 5 { assert!(root.pack(&nodes[cur]).is_some()); }
		else if root.pack(&nodes[cur]).is_none() { println!("Filled at {} nodes", cur); break; }
		cur += 1;
	}
}

#[test]
fn texture_atlas() {
	
}
