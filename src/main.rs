extern crate sdl2;
extern crate sdl2_ttf;
extern crate time;

use std::path::Path;

use sdl2::event::{Event};
use sdl2::surface::{Surface};
use sdl2::pixels::Color;

use std::ops::Add;


struct Point<T: Add>{
    x: T,
    y: T
}

struct Entity{
    id: i32,
    position: Point<f32>,
    direction: f32,
    speed: f32
}

impl Entity{
    fn new(id: i32, position: (f32, f32)) -> Entity{
        Entity{
            id:id,
            position:
                Point{
                    x:position.0,
                    y:position.1
                },
            direction:0.0,
            speed:0.0
        }
    }

    fn update(&mut self){
        self.position.x += self.direction.cos() * self.speed;
        self.position.y += self.direction.sin() * self.speed;
    }
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let ttf_ctx = sdl2_ttf::init().unwrap();

    let window  = match video_ctx.window("Aries", 800, 600).position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let mut renderer = match window.renderer().accelerated().present_vsync().build() {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let font = match ttf_ctx.load_font(Path::new("font.ttf"), 16){
        Ok(font) => font,
        Err(err) => panic!("failed to load font: {}", err)
    };

    let mut events = ctx.event_pump().unwrap();

    let mut tdiff = time::now();
    'event : loop {
        renderer.clear();

        let timespec = time::now()-tdiff;
        let millistxt = format!("{}", (1000*1000)/timespec.num_microseconds().unwrap());
        tdiff = time::now();

        let surface = font.render(&millistxt)
            .blended(Color::RGBA(255, 0, 0, 255)).unwrap();
        let mut texture = renderer.create_texture_from_surface(&surface).unwrap();

        renderer.copy(&texture, None, Some(surface.rect()));

        renderer.present();


        for event in events.poll_iter() { match event { Event::Quit{..} => break 'event, _ => continue } }
    }
}
