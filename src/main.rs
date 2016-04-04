#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate sdl2;
extern crate sdl2_ttf;
extern crate sdl2_gfx;
extern crate time;

extern crate serde;
extern crate bincode;

use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;

use sdl2::event::Event;
use sdl2::pixels::Color;

use sdl2::render::Renderer;

use time::Tm;

mod game;

use game::grid::Grid;
use game::network::Network;
use game::entity::EntityType::*;

fn draw_fps(prev: &mut Tm, renderer: &mut Renderer, font: &sdl2_ttf::Font) {
    let timespec = time::now() - *prev;
    let millistxt = format!("{}", (1000 * 1000) / timespec.num_microseconds().unwrap());
    *prev = time::now();

    let surface = font.render(&millistxt)
                      .blended(Color::RGBA(255, 0, 0, 255))
                      .unwrap();
    let texture = renderer.create_texture_from_surface(&surface).unwrap();
    renderer.copy(&texture, None, Some(surface.rect()));
}

fn main() {
    // Initialize contexts for SDL
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let mouse_ctx = ctx.mouse();
    let ttf_ctx = sdl2_ttf::init().unwrap();

    let font = ttf_ctx.load_font(Path::new("font.ttf"), 16).unwrap();

    let window = video_ctx.window("Aries", 800, 600).position_centered().opengl().build().unwrap();
    let mut renderer = window.renderer().accelerated().present_vsync().build().unwrap();

	let mut network = Network::new("0.0.0.0:6666");

	let (tx, rx) = channel();
	thread::spawn(move|| network.start("127.0.0.1:6665",tx));


    // Initialize grid
    let mut grid = Grid::new(25);

	grid.new_entity(Unit, 0, 0, 0);

	/*for i in 1..32 {
		for j in 1..24 {
			let id = grid.new_entity(Unit, 0, 0, 0);
			grid.move_entity(id, i, j);
		}
	}*/

    for i in 1..20 {
        for j in 15..18 {
            grid.new_entity(Building, j, i, 1);
        }
    }
    for i in 18..20 {
        for j in 2..5 {
            grid.new_entity(Building, j, i, 0);
        }
    }


    // Variables for event and FPS counting
    let mut events = ctx.event_pump().unwrap();
    let mut tdiff = time::now();

    'event: loop {
        renderer.set_draw_color(Color::RGB(0, 50, 100));
        renderer.clear();

		match rx.try_recv() {
			Ok(cs) => grid.execute(cs),
			_ => {}
		}

        draw_fps(&mut tdiff, &mut renderer, &font);

        grid.render(&mut renderer);
        grid.update();

        let (button, mx, my) = mouse_ctx.mouse_state();
        grid.set_highlight((mx as f32 / 25.0).round() as i32,
                           (my as f32 / 25.0).round() as i32,
                           button.left());
		if button.left() {
			for i in 0..1{
				grid.move_entity_to_highlight(i);
			}
		}
        renderer.present();

        // Poll for events
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'event,
                Event::MouseButtonUp { .. } => {
                    // grid.move_entity_to_highlight(0);
                }
                _ => continue,
            }
        }
    }
}
