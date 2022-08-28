use glutin::event::{Event, WindowEvent};
use glutin::event_loop::*;

pub mod graphics;

// main
fn main() {
	unsafe {
		let event_loop: EventLoop<()> = EventLoop::new();
		let window = glutin::ContextBuilder::new()
				.build_windowed(
					glutin::window::WindowBuilder::new().with_title("tetris").with_inner_size(glutin::dpi::LogicalSize::new(600., 400.)), &event_loop
				).unwrap().make_current().unwrap();

		let mut g: Box<dyn graphics::api::api::GraphicsAPI> = Box::<graphics::api::gl::GLContext>::new(graphics::api::gl::GLContext::new(&window));
		g.setup();
		

		event_loop.run(move |event, _, control_flow| {
			*control_flow = ControlFlow::Wait;
			match event {
				Event::LoopDestroyed => { return; }
				Event::MainEventsCleared => { window.window().request_redraw(); }
				Event::RedrawRequested(_) => {
					window.swap_buffers().unwrap();
				}
				Event::WindowEvent { ref event, .. } => match event {
					WindowEvent::Resized(physical_size) => {
						g.draw();
						window.resize(*physical_size);
					}
					WindowEvent::CloseRequested => {
						g.destroy();
						*control_flow = ControlFlow::Exit
					}
					_ => (),
				},
				_ => (),
			}
		});
	}
}

