use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 600.0;
const PLAYER_SIZE: f32 = 16.0;

struct Shape {
    x: f32,
    y: f32, 
    size: f32, 
    speed: f32, 
    color: Color
}

impl Shape {
    fn draw(&self) {
        draw_circle(self.x, self.y, self.size, self.color);
    }
}



#[macroquad::main("Space Shooter")]
async fn main() {

    let mut player = Shape {
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        size: PLAYER_SIZE, 
        speed: PLAYER_SPEED, 
        color: YELLOW
    };





    loop {
        clear_background(DARKPURPLE);

        let delta = get_frame_time();
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

        player.x = player.x.clamp(player.size / 2.0, screen_width() - player.size/2.0);
        player.y = player.y.clamp(player.size /2.0, screen_height() - player.size /2.0);

        player.draw();
        next_frame().await;
    }
}

