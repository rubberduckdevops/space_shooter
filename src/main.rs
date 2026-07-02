use macroquad::prelude::*;
use macroquad::rand::gen_range;

const PLAYER_SPEED: f32 = 600.0;
const PLAYER_SIZE: f32 = 16.0;

#[derive(Debug)]
struct Shape {
    x: f32,
    y: f32,
    size: f32,
    speed: f32,
    color: Color,
    collided: bool,
}

impl Shape {
    fn draw(&self) {
        draw_circle(self.x, self.y, self.size, self.color);
    }

    fn collides_with(&self, other: &Shape) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < (self.size / 2.0 + other.size / 2.0) * 1.25
    }
}

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
    let mut player = Shape {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        size: PLAYER_SIZE / 2.0,
        speed: PLAYER_SPEED,
        color: YELLOW,
        collided: false,
    };

    let mut score = 0;
    let mut bullets: Vec<Shape> = Vec::new();

    let mut enemies: Vec<Shape> = Vec::new();

    let mut spawn_timer = 0.0;
    let spawn_interval = 0.5;

    let mut game_state = GameState::MainMenu;

    loop {
        clear_background(DARKPURPLE);

        match game_state {
            GameState::MainMenu => {
                draw_centered_text("Space Shooter", 200.0, 60, YELLOW);
                draw_centered_text("Press Any Key To play!", 280.0, 30, WHITE);
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                let delta = get_frame_time();

                // Player Actions and Movement
                if is_key_down(KeyCode::Right) {
                    player.x += player.speed * delta;
                }
                if is_key_down(KeyCode::Left) {
                    player.x -= player.speed * delta;
                }
                if is_key_down(KeyCode::Down) {
                    player.y += player.speed * delta;
                }
                if is_key_down(KeyCode::Up) {
                    player.y -= player.speed * delta;
                }
                if is_key_pressed(KeyCode::Space) {
                    bullets.push(Shape {
                        x: player.x,
                        y: player.y,
                        size: 16.0 / 2.0,
                        speed: 500.0,
                        color: RED,
                        collided: false,
                    });
                }

                // Moving the bullets!
                for bullet in bullets.iter_mut() {
                    bullet.y -= bullet.speed * delta;
                }
                for bullet in bullets.iter() {
                    bullet.draw();
                }

                // Lock Player in Frame
                player.x = player
                    .x
                    .clamp(player.size / 2.0, screen_width() - player.size / 2.0);
                player.y = player
                    .y
                    .clamp(player.size / 2.0, screen_height() - player.size / 2.0);

                player.draw();

                // Enemies
                spawn_timer += delta;
                if spawn_timer > spawn_interval {
                    spawn_timer = 0.0;

                    let enemie_size = gen_range(20.0, 45.0);

                    enemies.push(Shape {
                        x: gen_range(enemie_size / 2.0, screen_width() - enemie_size / 2.0), //Variable enemy position
                        y: -enemie_size,
                        size: enemie_size / 2.0,
                        speed: gen_range(80.0, 120.0), // Variable Enemy Speed
                        color: RED,
                        collided: false,
                    });
                }

                // Move Enemies
                for enemy in enemies.iter_mut() {
                    enemy.y += enemy.speed * delta;
                }
                // Clean Up Enemies
                for enemy in enemies.iter() {
                    enemy.draw()
                }

                for enemy in enemies.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if enemy.collides_with(bullet) {
                            println!("Collison Detected");
                            enemy.collided = true;
                            bullet.collided = true;
                            score += enemy.size as u32;
                        }
                    }
                }

                // Clean Up
                enemies.retain(|e| !e.collided && e.y - e.size / 2.0 < screen_height());
                bullets.retain(|b| !b.collided && b.y + b.size / 2.0 > 0.0);

                if enemies.iter().any(|e| player.collides_with(e)) {
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
                    player.x = screen_width() / 2.0;
                    player.y = screen_height() - 60.0;
                    score = 0;
                    spawn_timer = 0.0;
                    game_state = GameState::Playing;
                }
            }
        }

        next_frame().await;
    }
}

