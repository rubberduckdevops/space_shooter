use std::env::{self, var};

use macroquad::prelude::*;

#[derive(Debug)]
pub enum ShapeType {
    Circle,
    Square,
    Custom,
}

#[derive(Debug)]
pub struct Shape {
    // ToDo: Should just make this a Vec2? 
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub hitbox: Vec2,
    pub speed: f32,
    pub color: Color,
    pub collided: bool,
    pub shape_type: ShapeType,
}

impl Shape {
    pub fn draw(&self) {
        match self.shape_type {
            ShapeType::Circle => {
                draw_circle(self.x, self.y, self.size, self.color);
            }
            ShapeType::Square => {
                draw_rectangle(self.x, self.y, self.size, self.size, self.color);
            }
            ShapeType::Custom => {
                // Do nothing here this is used for asset drawing.
                if env::var("SHOW_HIT_BOX").is_ok() {
                    draw_rectangle_lines(self.x, self.y, self.hitbox.x, self.hitbox.y, 2.0, GREEN);
                }
            }
        }
    }

    pub fn rect(&self) -> Rect {
        match self.shape_type {
            ShapeType::Circle => {
                // Circle draws from center, so offset to get top-left
                Rect::new(
                    self.x - self.size,
                    self.y - self.size,
                    self.size * 2.0,
                    self.size * 2.0,
                )
            }
            ShapeType::Square => Rect::new(self.x, self.y, self.size, self.size),
            ShapeType::Custom => {
                // Use hitbox vec2 for width/height (set this to texture size)
                Rect::new(self.x, self.y, self.hitbox.x, self.hitbox.y)
            }
        }
    }

    pub fn collides_with(&self, other: &Shape) -> bool {
        self.rect().overlaps(&other.rect())
    }
}

pub struct Explosion {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub timer: f32,
    pub frame_count: usize,
}

impl Explosion {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Explosion {
            x,
            y,
            size,
            timer: 0.0,
            frame_count: 12,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.timer += delta;
    }

    pub fn is_done(&self) -> bool {
        self.timer > 1.2 // half second duration, tweak as needed
    }

      pub fn current_frame(&self) -> usize {
          let frame = (self.timer / 0.1) as usize;
          frame.min(self.frame_count - 1) // clamp to last frame
      }

    pub fn draw(&self, asset: &[Texture2D]) {
        log::trace!("EXPLOSION! {},{}", self.x, self.y);

        let params = DrawTextureParams {
            dest_size: Some(vec2(self.size, self.size)),
            ..Default::default()
        };
        draw_texture_ex(
            &asset[self.current_frame()],
            self.x - self.size / 2.0, // center it on the impact point
            self.y - self.size / 2.0,
            WHITE,
            params,
        );
    }
}
