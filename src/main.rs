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
    env_logger::init();
    let mut player = Shape {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        size: PLAYER_SIZE / 2.0,
        speed: PLAYER_SPEED,
        color: YELLOW,
        collided: false,
    };

    log::info!("Loading Assets");
    let player_ship = load_texture("assets/spacecow.png").await.unwrap();
    player_ship.set_filter(FilterMode::Nearest);

    let player_ship_preview = load_texture("assets/spacecow_preview.png").await.unwrap();
    let milk_rocket = load_texture("assets/milk_bolt.png").await.unwrap();
    let enemy_ship = load_texture("assets/ufo.png").await.unwrap();
    enemy_ship.set_filter(FilterMode::Nearest);

    log::info!("Loading Assets Done");

    let mut score = 0;
    let mut bullets: Vec<Shape> = Vec::new();

    let mut enemies: Vec<Shape> = Vec::new();

    let mut spawn_timer = 0.0;
    let spawn_interval = 0.5;

    let mut game_state = GameState::MainMenu;

    log::info!("Starting Main Loop");
    loop {
        clear_background(DARKPURPLE);

        match game_state {
            GameState::MainMenu => {
                draw_centered_text("Cow Space Shooter", 200.0, 60, YELLOW);
                draw_centered_text("Press SPACE To play!", 280.0, 30, WHITE);
                draw_texture(&player_ship_preview, screen_width() / 2.0 - 120.0, 300.0, WHITE);
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
                if is_key_pressed(KeyCode::Escape) {
                        game_state = GameState::MainMenu;
                }

                // Moving the bullets!
                for bullet in bullets.iter_mut() {
                    bullet.y -= bullet.speed * delta;
                }
                for bullet in bullets.iter() {
                    bullet.draw();
                    let diamater = bullet.size * 2.0;
                    let params = DrawTextureParams {
                        dest_size: Some(vec2(diamater, diamater)),
                        ..Default::default()
                    };
                    draw_texture_ex(
                        &milk_rocket,
                        bullet.x - diamater / 2.0,
                        bullet.y - diamater / 2.0,
                        WHITE,
                        params,
                    );
                }

                // Lock Player in Frame
                player.x = player
                    .x
                    .clamp(player.size / 2.0, screen_width() - player.size / 2.0);
                player.y = player
                    .y
                    .clamp(player.size / 2.0, screen_height() - player.size / 2.0);

                player.draw();
                // Draw the space Cow right after the circle
                // -16 seems to center the cow on the dot pretty well
                // better might be some actual math
                // similar to how enemy is drawn... but don't break something that works?
                draw_texture(&player_ship, player.x - 16.0, player.y - 16.0, WHITE);

                // Enemies
                spawn_timer += delta;
                if spawn_timer > spawn_interval {
                    log::debug!("Spawning Enemy");
                    spawn_timer = 0.0;

                    let enemie_size = gen_range(45.0, 75.0);

                    enemies.push(Shape {
                        x: gen_range(enemie_size / 2.0, screen_width() - enemie_size / 2.0), //Variable enemy position
                        y: -enemie_size,
                        size: enemie_size / 2.0,
                        speed: gen_range(80.0, 120.0), // Variable Enemy Speed
                        color: LIGHTGRAY,              // Color of "shields"??
                        collided: false,
                    });
                }

                // Move Enemies
                for enemy in enemies.iter_mut() {
                    enemy.y += enemy.speed * delta;
                }
                for enemy in enemies.iter() {
                    enemy.draw();
                    let diamater = enemy.size * 2.0;
                    let params = DrawTextureParams {
                        dest_size: Some(vec2(diamater, diamater)),
                        ..Default::default()
                    };
                    draw_texture_ex(
                        &enemy_ship,
                        enemy.x - diamater / 2.0,
                        enemy.y - diamater / 2.0,
                        WHITE,
                        params,
                    );
                }

                for enemy in enemies.iter_mut() {
                    for bullet in bullets.iter_mut() {
                        if enemy.collides_with(bullet) {
                            log::debug!("Bullet Collision with Enemy");
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
