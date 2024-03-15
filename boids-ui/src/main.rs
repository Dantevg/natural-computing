use boids::{world::World, Params};
use pixels::{Pixels, SurfaceTexture};
use winit::{
	dpi::PhysicalSize,
	event::{Event, WindowEvent},
	event_loop::{ControlFlow, EventLoop},
	window::WindowBuilder,
};

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn main() {
	let params = Params {
		alignment_strength: 0.1,
		cohesion_strength: 0.005,
		separation_strength: 1.0,
	};
	let mut world: World<300> = World::new(WIDTH, HEIGHT, params);

	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Wait);

	let window = WindowBuilder::new()
		.with_title("Boids")
		.with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
		.build(&event_loop)
		.unwrap();

	let window_size = window.inner_size();
	let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
	let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

	event_loop
		.run(move |event, window_target| {
			if let Event::WindowEvent { event, .. } = event {
				match event {
					WindowEvent::CloseRequested => window_target.exit(),
					WindowEvent::RedrawRequested => {
						world.update(0.01);
					}
					_ => (),
				}
			}
		})
		.unwrap();
}
