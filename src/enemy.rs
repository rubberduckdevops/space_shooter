use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::common::{Shape, ShapeType};

#[derive(Debug)]
pub struct BasicEnemy {
    pub shape: Shape,
}

impl BasicEnemy {
    pub fn new() -> Self {
        let size = gen_range(45.0, 75.0);
        BasicEnemy {
            shape: Shape {
                x: gen_range(0.0, screen_width() - size),
                y: -size,
                size,
                hitbox: vec2(size, size),
                speed: gen_range(80.0, 120.0),
                color: DARKPURPLE,
                collided: false,
                shape_type: ShapeType::Custom,
            },
        }
    }

    pub fn move_down(&mut self, delta: f32) {
        self.shape.y += self.shape.speed * delta;
    }

    pub fn draw(&self, asset: &Texture2D) {
        let params = DrawTextureParams {
            dest_size: Some(vec2(self.shape.size, self.shape.size)),
            ..Default::default()
        };
        draw_texture_ex(asset, self.shape.x, self.shape.y, WHITE, params);
    }
}
