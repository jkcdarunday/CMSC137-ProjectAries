extern crate sdl2;
extern crate sdl2_gfx;

use sdl2::render::Renderer;
use sdl2::pixels::Color;

use std::f32::consts::PI;

use sdl2_gfx::primitives::DrawRenderer;

use game::point::Point;

pub enum EntityType {
    Unit,
    Building,
}

// TODO: Target grid location : DONE
// TODO: Grid : DONE
// TODO: Buildings: PARTIAL
// TODO: Faction : DONE
#[allow(dead_code)]
pub struct Entity {
    e_type: EntityType,
    grid_position: Point<i32>,
    position: Point<f32>,
    direction: f32,
    speed: f32,
    grid_multiplier: i32,
    life: f32,
    faction: i32,
}

impl Entity {
    pub fn new(e_type: EntityType, x: i32, y: i32, faction: i32, grid_multiplier: i32) -> Entity {
        Entity {
            e_type: e_type,
            grid_position: Point { x: x, y: y },
            position: Point {
                x: (x * grid_multiplier) as f32,
                y: (y * grid_multiplier) as f32,
            },
            grid_multiplier: grid_multiplier,
            direction: 0.0,
            speed: 2.0,
            life: 0.9,
            faction: faction,
        }
    }

    fn angle(xo: f32, yo: f32, xe: f32, ye: f32) -> f32 {
        let dx = xe - xo;
        let dy = ye - yo;
        let mut a = dy.atan2(dx);
        while a < 0.0 {
            a += PI * 2.0;
        }
        a
    }

    #[allow(dead_code)]
    pub fn direction(&mut self, d: f32) {
        self.direction = d;
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.grid_position.x = x;
        self.grid_position.y = y;
    }

    pub fn update(&mut self) {
        match self.e_type {
            EntityType::Unit => {
                let dist = self.position
                               .manhattan(self.grid_position.multiply(self.grid_multiplier as f32));
                if dist > 1.0{
                    self.direction = Entity::angle(
						self.position.x, self.position.y,
						(self.grid_position.x*self.grid_multiplier) as f32,
						(self.grid_position.y*self.grid_multiplier) as f32
					).to_degrees();
                    /*self.position.x += self.direction.to_radians().cos() * self.speed;
                      self.position.y += self.direction.to_radians().sin() * self.speed;*/
                    self.position.x += ((self.grid_position.x*self.grid_multiplier) as f32 - self.position.x)/20.0;
                    self.position.y += ((self.grid_position.y*self.grid_multiplier) as f32 - self.position.y)/20.0;
                } else {
                    self.position.x = (self.grid_position.x*self.grid_multiplier) as f32;
                    self.position.y = (self.grid_position.y*self.grid_multiplier) as f32;
                }
            }
            EntityType::Building => {
                // TODO
            }
        }
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        // Render life
        for i in 8..10 {
            renderer.arc(self.position.x as i16,
                         self.position.y as i16,
                         i,
                         self.direction as i16,
                         (self.direction + self.life * 360.0) as i16,
                         Color::RGB(180, 0, 0))
                    .unwrap();
        }
        match self.e_type {
            EntityType::Unit => {
                // Render life
                for i in 8..10 {
                    renderer.arc(self.position.x as i16,
                                 self.position.y as i16,
                                 i,
                                 self.direction as i16,
                                 (self.direction + self.life * 360.0) as i16,
                                 Color::RGB(180, 0, 0))
                            .unwrap();
                }
                // Render circular container
                let mut faction_color = Color::RGB(0, 255, 0);
                if self.faction > 0 {
                    faction_color = Color::RGB(255, 0, 0);
                }
                renderer.circle(self.position.x as i16,
                                self.position.y as i16,
                                6,
                                faction_color)
                        .unwrap();

                // Render directional arrow
                renderer.arc(self.position.x as i16,
                             self.position.y as i16,
                             10,
                             self.direction as i16 - 10,
                             self.direction as i16 + 10,
                             Color::RGB(0, 255, 0))
                        .unwrap();
                renderer.arc(self.position.x as i16,
                             self.position.y as i16,
                             11,
                             self.direction as i16 - 6,
                             self.direction as i16 + 6,
                             Color::RGB(0, 255, 0))
                        .unwrap();
                renderer.arc(self.position.x as i16,
                             self.position.y as i16,
                             12,
                             self.direction as i16 - 2,
                             self.direction as i16 + 2,
                             Color::RGB(0, 255, 0))
                        .unwrap();
            }
            EntityType::Building => {
                let mut faction_color = Color::RGBA(0, 255, 0, 100);
                if self.faction > 0 {
                    faction_color = Color::RGBA(255, 255, 255, 100);
                }
                let offset = (self.grid_multiplier / 2) as i16;
                renderer.rectangle(self.position.x as i16 - offset,
                                   self.position.y as i16 - offset,
                                   self.position.x as i16 + offset,
                                   self.position.y as i16 + offset,
                                   faction_color)
                        .unwrap();
            }
        }
    }
}
