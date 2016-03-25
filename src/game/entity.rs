extern crate sdl2;
extern crate sdl2_gfx;

use std::ops::Add;
use sdl2::render::Renderer;
use sdl2::pixels::Color;

use sdl2_gfx::primitives::DrawRenderer;

pub struct Point<T: Add>{
    x: T,
    y: T
}

#[allow(dead_code)]
pub struct Entity{
    id: i32,
    position: Point<f32>,
    direction: f32,
    speed: f32,
    life: f32
}

impl Entity{
    pub fn new(id: i32, position: (f32, f32)) -> Entity{
        Entity{
            id:id,
            position:
                Point{
                    x:position.0,
                    y:position.1
                },
            direction:0.0,
            speed:2.0,
            life: 0.9
        }
    }
    pub fn direction(&mut self, d: f32){
        self.direction = d;
    }

    pub fn update(&mut self){
        self.position.x += self.direction.to_radians().cos() * self.speed;
        self.position.y += self.direction.to_radians().sin() * self.speed;
        self.direction += 2.0;
        self.life=self.direction/10.0;
    }

    pub fn render(&mut self, renderer: &mut Renderer){
        for i in 10..12 {
            renderer.arc(self.position.x as i16, self.position.y as i16, i, self.direction as i16, (self.direction+self.life*360.0) as i16, Color::RGB(180,0,0)).unwrap();
        }
        renderer.circle(self.position.x as i16, self.position.y as i16, 8, Color::RGB(255,255,255)).unwrap();
        renderer.arc(self.position.x as i16, self.position.y as i16, 13, self.direction as i16-15, self.direction as i16+15, Color::RGB(0,255,0)).unwrap();
        renderer.arc(self.position.x as i16, self.position.y as i16, 14, self.direction as i16-10, self.direction as i16+10, Color::RGB(0,255,0)).unwrap();
        renderer.arc(self.position.x as i16, self.position.y as i16, 15, self.direction as i16-5, self.direction as i16+5, Color::RGB(0,255,0)).unwrap();
    }
}
