pub mod common;
pub mod enemy;
pub mod player;

use crate::common::{Explosion, Shape};
use crate::enemy::BasicEnemy;
use crate::player::Player;
use macroquad::prelude::*;

fn draw_centered_text(text: &str, y: f32, font_size: u16, color: Color) {
    let dims = measure_text(text, None, font_size, 1.0);
    let x = screen_width() / 2.0 - dims.width / 2.0;
    draw_text(text, x, y, font_size as f32, color);
}

enum GameState {
    MainMenu,
    Playing,
    GameOver,
}

#[macroquad::main("Space Shooter")]
async fn main() {
    env_logger::init();

    log::info!("Initializing Player");
    let mut player = Player::init().await;

    log::info!("Loading Assets");
    let player_ship = load_texture("assets/eagle.png").await.unwrap();
    player_ship.set_filter(FilterMode::Nearest);

    let player_ship_preview = load_texture("assets/eagle_preview.png").await.unwrap();

    // Due to the spawn rate we load this here an not in the new struct function
    let enemy_ship = load_texture("assets/teapot.png").await.unwrap();
    enemy_ship.set_filter(FilterMode::Nearest);
    let mut explosion_frames: Vec<Texture2D> = Vec::new();
    for i in 1..=12 {
        let path = format!("assets/explosion/expl_04_{:04}.png", i);
        let tex = load_texture(&path).await.unwrap();
        tex.set_filter(FilterMode::Nearest);
        explosion_frames.push(tex);
    }

    log::info!("Loading Assets Done");

    let mut score = 0;
    let mut bullets: Vec<Shape> = Vec::new();
    let mut enemies: Vec<BasicEnemy> = Vec::new();
    let mut explosions: Vec<Explosion> = Vec::new();
    let mut spawn_timer = 0.0;
    let spawn_interval = 0.5;

    let mut game_state = GameState::MainMenu;

    log::info!("Starting Main Loop");
    loop {
        clear_background(DARKPURPLE);

        match game_state {
            GameState::MainMenu => {
                draw_centered_text("FREEDOM EDITION", 200.0, 60, YELLOW);
                draw_centered_text("Press SPACE To play!", 280.0, 30, WHITE);
                draw_texture(
                    &player_ship_preview,
                    screen_width() / 2.0 - 120.0,
                    300.0,
                    WHITE,
                );
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                let delta = get_frame_time();

                // Player Actions and Movement
                if is_key_down(KeyCode::Right) {
                    player.move_right(delta);
                }
                if is_key_down(KeyCode::Left) {
                    player.move_left(delta);
                }
                if is_key_down(KeyCode::Down) {
                    player.move_down(delta);
                }
                if is_key_down(KeyCode::Up) {
                    player.move_up(delta);
                }
                if is_key_pressed(KeyCode::Space) {
                    player.fire();
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::MainMenu;
                }

                player.move_bullets(delta);
                player.draw_bullets();
                player.draw();

                // Enemies
                spawn_timer += delta;
                if spawn_timer > spawn_interval {
                    log::debug!("Spawning Enemy");
                    // Todo Create levels?
                    spawn_timer = 0.0;
                    enemies.push(BasicEnemy::new());
                }

                // Move Enemies
                for enemy in enemies.iter_mut() {
                    enemy.move_down(delta);
                }
                for enemy in enemies.iter() {
                    enemy.draw(&enemy_ship);
                }

                for enemy in enemies.iter_mut() {
                    for bullet in player.bullets.iter_mut() {
                        if enemy.shape.collides_with(bullet) {
                            log::debug!("Bullet Collision with Enemy");
                            let explosion = Explosion::new(enemy.shape.x, enemy.shape.y, 32.0);
                            explosions.push(explosion);
                            enemy.shape.collided = true;
                            bullet.collided = true;
                            score += 1;
                        }
                    }
                }

                explosions.iter_mut().for_each(|e| {
                    e.draw(&explosion_frames);
                    e.update(delta);
                });

                // Clean Up
                explosions.retain(|e| !e.is_done());
                enemies.retain(|e| {
                    !e.shape.collided && e.shape.y - e.shape.size / 2.0 < screen_height()
                });
                player
                    .bullets
                    .retain(|b| !b.collided && b.y + b.size / 2.0 > 0.0);

                if enemies.iter().any(|e| player.shape.collides_with(&e.shape)) {
                    log::info!("Player was hit by enemy, game over");
                    game_state = GameState::GameOver;
                }

                // In Game stats
                draw_text(
                    &format!("Bullets: {}", bullets.len()),
                    10.0,
                    30.0,
                    25.0,
                    WHITE,
                );
                draw_text(
                    &format!("Enemies: {}", enemies.len()),
                    10.0,
                    60.0,
                    25.0,
                    WHITE,
                );
                draw_text(&format!("Score: {}", score), 10.0, 90.0, 25.0, WHITE);
            }
            GameState::GameOver => {
                draw_centered_text("GAME OVER", 220.0, 60, RED);
                draw_centered_text(&format!("Score: {}", score), 290.0, 40, WHITE);
                draw_centered_text("Press SPACE to play again", 350.0, 28, WHITE);

                if is_key_pressed(KeyCode::Space) {
                    // reset everything for a fresh run
                    bullets.clear();
                    enemies.clear();
                    player.shape.x = screen_width() / 2.0;
                    player.shape.y = screen_height() - 60.0;
                    score = 0;
                    spawn_timer = 0.0;
                    game_state = GameState::Playing;
                }
            }
        }

        next_frame().await;
    }
}
