use macroquad::prelude::*;

use crate::common::{Shape, ShapeType};

const PLAYER_SPEED: f32 = 600.0;
const PLAYER_SIZE: f32 = 16.0;

pub struct Player {
    pub shape: Shape,
    pub bullets: Vec<Shape>,
    pub player_asset: Texture2D,
    pub bullet_asset: Texture2D,
}

impl Player {
    pub async fn init() -> Self {
        let player_asset = load_texture("assets/eagle.png").await.unwrap();
        let bullet_asset = load_texture("assets/firework_rocket.png").await.unwrap();
        log::debug!(
            "Player Asset: {}x{}",
            player_asset.width(),
            player_asset.height()
        );
        log::debug!(
            "Bullet Asset: {}x{}",
            bullet_asset.width(),
            bullet_asset.height()
        );

        Player {
            shape: Shape {
                x: screen_width() / 2.0,
                y: screen_height() / 2.0,
                size: PLAYER_SIZE / 2.0,
                // Modify the Player hit box and shrink by 20%
                hitbox: vec2(player_asset.width() * 0.7 , player_asset.height() * 0.7),
                speed: PLAYER_SPEED,
                color: YELLOW,
                collided: false,
                shape_type: ShapeType::Custom,
            },
            bullets: Vec::new(),
            player_asset: player_asset,
            bullet_asset: bullet_asset,
        }
    }
    pub fn move_right(&mut self, delta: f32) {
        self.shape.x += self.shape.speed * delta;
    }
    pub fn move_left(&mut self, delta: f32) {
        self.shape.x -= self.shape.speed * delta;
    }
    pub fn move_up(&mut self, delta: f32) {
        self.shape.y -= self.shape.speed * delta;
    }
    pub fn move_down(&mut self, delta: f32) {
        self.shape.y += self.shape.speed * delta;
    }

    pub fn draw(&mut self) {
        // Lock Player in Frame
        self.shape.x = self.shape.x.clamp(
            self.shape.size / 2.0,
            screen_width() - self.shape.size / 2.0,
        );
        self.shape.y = self.shape.y.clamp(
            self.shape.size / 2.0,
            screen_height() - self.shape.size / 2.0,
        );
        self.shape.draw();
        draw_texture(&self.player_asset, self.shape.x, self.shape.y, WHITE);
    }

    pub fn fire(&mut self) {
        let player_width = self.player_asset.width();

        let new_bullet = Shape {
            x: self.shape.x + player_width / 2.0 - self.bullet_asset.width() / 2.0,
            y: self.shape.y,
            hitbox: vec2(self.bullet_asset.width(), self.bullet_asset.height()),
            size: self.bullet_asset.width() / 2.0,
            speed: 500.0,
            color: RED,
            collided: false,
            shape_type: ShapeType::Custom,
        };
        self.bullets.push(new_bullet);
    }

    pub fn move_bullets(&mut self, delta: f32) {
        for bullet in self.bullets.iter_mut() {
            bullet.y -= bullet.speed * delta;
        }
    }

    pub fn draw_bullets(&self) {
        for bullet in self.bullets.iter() {
            // Leaving this here but it will not do anything since
            // since we are using ShapeType::Custom
            bullet.draw();
            let diamater = bullet.size * 2.0;
            let params = DrawTextureParams {
                // This would make a 16x16 hitbox based on the size of
                dest_size: Some(vec2(diamater, diamater)),
                ..Default::default()
            };
            draw_texture_ex(&self.bullet_asset, bullet.x, bullet.y, WHITE, params);
        }
    }
}
