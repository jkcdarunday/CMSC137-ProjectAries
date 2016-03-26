extern crate sdl2;
extern crate sdl2_ttf;
extern crate sdl2_gfx;
extern crate time;

use std::path::Path;

use sdl2::event::{Event};
use sdl2::pixels::Color;

use sdl2::render::Renderer;
use sdl2::rect::Point;

use sdl2_gfx::primitives::DrawRenderer;
use time::Tm;

mod game{
    pub mod entity;
}
use game::entity::Entity;

fn draw_fps(prev: &mut Tm, renderer: &mut Renderer, font: &sdl2_ttf::Font){
    let timespec = time::now()-*prev;
    let millistxt = format!("{}", (1000*1000)/timespec.num_microseconds().unwrap());
    *prev = time::now();

    let surface = font.render(&millistxt)
        .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
    let texture = renderer.create_texture_from_surface(&surface).unwrap();
    renderer.copy(&texture, None, Some(surface.rect()));
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let ttf_ctx = sdl2_ttf::init().unwrap();

    let window  = video_ctx.window("Aries", 800, 600).position_centered().opengl().build().unwrap();
    let mut renderer = window.renderer().accelerated().present_vsync().build().unwrap();
    let font = ttf_ctx.load_font(Path::new("font.ttf"), 16).unwrap();

    let mut events = ctx.event_pump().unwrap();

    let mut tdiff = time::now();

    let mut ev = Vec::<Entity>::new();
    for i in 1..2{
		let mut e = Entity::new(0, 5, i, 25);
		e.move_to(9,i+3);
        ev.push(e);
    }

    'event : loop {
        renderer.set_draw_color(Color::RGB(0,50,100));
        renderer.clear();

        draw_fps(&mut tdiff, &mut renderer, &font);

        for e in &mut ev.iter_mut(){
            e.render(&mut renderer);
            e.update();
        }

        renderer.present();

        for event in events.poll_iter() { match event { Event::Quit{..} => break 'event, _ => continue } }
    }
}
