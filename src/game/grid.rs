extern crate sdl2;
extern crate sdl2_gfx;

use sdl2::render::Renderer;
use sdl2::pixels::Color;


use sdl2_gfx::primitives::DrawRenderer;

use game::entity::{Entity, EntityType};

pub struct Grid{
	grid_multiplier: i32,
	entities: Vec<Entity>
}

impl Grid{
	pub fn new(grid_multiplier: i32) -> Grid{
		Grid{
			grid_multiplier: grid_multiplier,
			entities: Vec::<Entity>::new()
		}
	}

	pub fn draw_grid(&mut self, renderer: &mut Renderer){
		for i in 1..(800/25){
			renderer.line(i*25, 0, i*25, 600, Color::RGBA(255,255,255,20)).unwrap();
		}

		for i in 1..(600/25){
			renderer.line(0, i*25, 800, i*25, Color::RGBA(255,255,255,20)).unwrap();
		}

		for x in 0..(800/25)+1{
			for y in 0..(600/25)+1{
				renderer.circle(x*25, y*25, 1, Color::RGBA(255,255,255,50)).unwrap();
			}
		}
	}

	pub fn update(&mut self){
		for e in &mut self.entities.iter_mut(){
			e.update();
		}
	}

	pub fn new_entity(&mut self, e_type:EntityType, x: i32, y: i32, faction: i32) -> usize{
		self.entities.push(Entity::new(e_type, x,y,faction,self.grid_multiplier));
		self.entities.len()-1
	}

	pub fn move_entity(&mut self, id: usize, x: i32, y: i32){
		self.entities.get_mut(id).unwrap().move_to(x,y);
	}

	pub fn render(&mut self, renderer: &mut Renderer){
		self.draw_grid(renderer);
		for e in self.entities.iter_mut(){
			e.render(renderer);
		}
	}
}
