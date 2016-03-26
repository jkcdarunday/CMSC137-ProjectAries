use std::ops::*;

pub struct Point<T: Add + Mul + Sub> {
    pub x: T,
    pub y: T,
}

impl Point<f32> {
    pub fn manhattan(&self, target: Point<f32>) -> f32 {
        (self.x - target.x).abs() + (self.y - target.y).abs()
    }

    #[allow(dead_code)]
    pub fn multiply(&self, value: f32) -> Point<f32> {
        Point {
            x: self.x * value,
            y: self.y * value,
        }
    }
}

impl Point<i32> {
    #[allow(dead_code)]
    pub fn manhattan(&self, target: Point<f32>) -> f32 {
        (self.x as f32 - target.x).abs() + (self.y as f32 - target.y).abs()
    }

    pub fn multiply(&self, value: f32) -> Point<f32> {
        Point {
            x: self.x as f32 * value,
            y: self.y as f32 * value,
        }
    }
}
