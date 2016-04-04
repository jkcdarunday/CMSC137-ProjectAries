extern crate sdl2;
extern crate sdl2_gfx;

use sdl2::render::Renderer;
use sdl2::pixels::Color;


use sdl2_gfx::primitives::DrawRenderer;

use game::entity::{Entity, EntityType};
use game::point::Point;
use game::network::Command::*;
use game::network::CommandList;

pub struct Grid {
    grid_multiplier: i32,
    entities: Vec<Entity>,
    highlight: (Point<i32>, bool),
}

impl Grid {
    pub fn new(grid_multiplier: i32) -> Grid {
        Grid {
            grid_multiplier: grid_multiplier,
            entities: Vec::<Entity>::new(),
            highlight: (Point { x: -1, y: -1 }, false),
        }
    }

    pub fn set_highlight(&mut self, x: i32, y: i32, clicked: bool) {
        self.highlight.0.x = x;
        self.highlight.0.y = y;
        self.highlight.1 = clicked;
    }

    pub fn draw_grid(&mut self, renderer: &mut Renderer) {
        for i in 1..(800 / 25) {
            renderer.line(i * 25, 0, i * 25, 600, Color::RGBA(255, 255, 255, 20)).unwrap();
        }

        for i in 1..(600 / 25) {
            renderer.line(0, i * 25, 800, i * 25, Color::RGBA(255, 255, 255, 20)).unwrap();
        }

        for x in 0..(800 / 25) + 1 {
            for y in 0..(600 / 25) + 1 {
                // Handle mouse highlight circle
                if self.highlight.0.x == x && self.highlight.0.y == y {
                    let mut size = 3;
                    if self.highlight.1 {
                        size = 5;
                    }
                    renderer.circle((x * 25) as i16,
                                    (y * 25) as i16,
                                    size,
                                    Color::RGBA(255, 255, 255, 100))
                            .unwrap();
                }
                renderer.circle((x * 25) as i16,
                                (y * 25) as i16,
                                1,
                                Color::RGBA(255, 255, 255, 50))
                        .unwrap();
            }
        }
    }

    pub fn update(&mut self) {
        for e in &mut self.entities.iter_mut() {
            e.update();
        }
    }

    pub fn new_entity(&mut self, e_type: EntityType, x: i32, y: i32, faction: i32) -> usize {
        self.entities.push(Entity::new(e_type, x, y, faction, self.grid_multiplier));
        self.entities.len() - 1
    }

    pub fn move_entity(&mut self, id: usize, x: i32, y: i32) {
        self.entities.get_mut(id).unwrap().move_to(x, y);
    }

    pub fn move_entity_to_highlight(&mut self, id: usize) {
        let x = self.highlight.0.x;
        let y = self.highlight.0.y;
        self.move_entity(id, x, y);
    }

	pub fn execute(&mut self, cs: CommandList){
		for c in cs.commands{
			match c{
				Move(id, x, y) => {self.move_entity(id as usize, x as i32, y as i32);},
				Life(id, life) => {},
				Face(id, x, y) => {},
			}
		}
	}

    pub fn render(&mut self, renderer: &mut Renderer) {
        self.draw_grid(renderer);
        for e in self.entities.iter_mut() {
            e.render(renderer);
        }
    }
}
