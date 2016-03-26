extern crate sdl2;
extern crate sdl2_gfx;

use std::ops::*;
use sdl2::render::Renderer;
use sdl2::pixels::Color;

use std::f32::consts::PI;

use sdl2_gfx::primitives::DrawRenderer;

pub struct Point<T: Add + Mul + Sub>{
    x: T,
    y: T
}

impl Point<f32>{
	pub fn manhattan(&self, target: Point<f32>) -> f32{
		(self.x-target.x).abs() + (self.y-target.y).abs()
	}

	pub fn multiply(&self, value: f32) -> Point<f32>{
		Point{
			x:self.x*value,
			y:self.y*value
		}
	}
}

impl Point<i32>{
	pub fn manhattan(&self, target: Point<f32>) -> f32{
		(self.x as f32-target.x).abs() + (self.y as f32-target.y).abs()
	}

	pub fn multiply(&self, value: f32) -> Point<f32>{
		Point{
			x:self.x as f32*value,
			y:self.y as f32*value
		}
	}
}



// TODO: Target grid location : DONE
// TODO: Grid
// TODO: Buildings
// TODO: Faction
#[allow(dead_code)]
pub struct Entity{
    id: i32,
    grid_position: Point<i32>,
    position: Point<f32>,
    direction: f32,
    speed: f32,
	grid_multiplier: i32,
    life: f32
}

impl Entity{
    pub fn new(id: i32, x: i32, y: i32, grid_multiplier: i32) -> Entity{
        Entity{
            id:id,
            grid_position:
                Point{
                    x:x,
                    y:y
                },
            position:
                Point{
                    x:(x*grid_multiplier) as f32,
                    y:(y*grid_multiplier) as f32
                },
				grid_multiplier: grid_multiplier,
            direction:0.0,
            speed:2.0,
            life: 0.9
        }
    }

    fn angle(xo: f32, yo: f32, xe: f32, ye: f32) -> f32{
        let dx = xe - xo;
        let dy = ye - yo;
        let mut a = dy.atan2(dx);
        while a < 0.0 {
            a += PI * 2.0;
        }
		a
    }

    pub fn direction(&mut self, d: f32){
        self.direction = d;
    }

	pub fn move_to(&mut self, x:i32, y:i32){
		self.grid_position.x = x;
		self.grid_position.y = y;
	}

    pub fn update(&mut self){
		let dist = self.position.manhattan(self.grid_position.multiply(self.grid_multiplier as f32));
		if dist > 1.0{
		       self.direction = Entity::angle(self.position.x, self.position.y, (self.grid_position.x*self.grid_multiplier) as f32, (self.grid_position.y*self.grid_multiplier) as f32).to_degrees();
			   /*self.position.x += self.direction.to_radians().cos() * self.speed;
			   self.position.y += self.direction.to_radians().sin() * self.speed;*/
			   self.position.x += ((self.grid_position.x*self.grid_multiplier) as f32 - self.position.x)/20.0;
			   self.position.y += ((self.grid_position.y*self.grid_multiplier) as f32 - self.position.y)/20.0;
		} else {
			self.position.x = (self.grid_position.x*self.grid_multiplier) as f32;
			self.position.y = (self.grid_position.y*self.grid_multiplier) as f32;
		}
    }

    pub fn render(&mut self, renderer: &mut Renderer){
		// Render life
        for i in 8..10 {
            renderer.arc(
				self.position.x as i16, self.position.y as i16, i,
				self.direction as i16, (self.direction+self.life*360.0) as i16,
				Color::RGB(180,0,0)
			).unwrap();
        }

		// Render circular container
        renderer.circle(
			self.position.x as i16, self.position.y as i16, 6,
			Color::RGB(255,255,255)
		).unwrap();

		// Render directional arrow
        renderer.arc(
			self.position.x as i16, self.position.y as i16, 10,
			self.direction as i16-10, self.direction as i16+10,
			Color::RGB(0,255,0)
		).unwrap();
        renderer.arc(
			self.position.x as i16, self.position.y as i16, 11,
			self.direction as i16-6, self.direction as i16+6,
			Color::RGB(0,255,0)
		).unwrap();
        renderer.arc(
			self.position.x as i16, self.position.y as i16, 12,
			self.direction as i16-2, self.direction as i16+2,
			Color::RGB(0,255,0)
		).unwrap();
    }
}
