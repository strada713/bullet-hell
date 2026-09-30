use macroquad::audio::{PlaySoundParams, load_sound, play_sound};
use macroquad::{
    color::{DARKBLUE, DARKGRAY, GREEN, PURPLE, RED, WHITE, YELLOW},
    conf::Conf,
    input::{self, KeyCode, is_key_down, is_key_pressed, is_key_released},
    main, miniquad,
    prelude::rand,
    shapes::{draw_circle, draw_triangle},
    text::draw_text,
    window::{clear_background, next_frame, screen_height, screen_width},
};

struct Ball {
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
    radius: f32,
    lives: i32,
}
impl Ball {
    fn move_ball(&mut self) {
        self.x += self.speed_x;
        self.y += self.speed_y;
    }
    fn check_walls(&mut self) {
        if self.x - self.radius < 0.0 || self.x + self.radius > screen_width() {
            self.speed_x = -self.speed_x;
        }
        if self.y - self.radius < 0.0 || self.y + self.radius > screen_height() {
            self.speed_y = -self.speed_y;
        }
    }
    fn handle_input(&mut self) {
        let mut current_speed = 7.0;

        if is_key_down(input::KeyCode::LeftShift) {
            current_speed = 3.5;
            self.radius = 10.0
        }
        if is_key_released(input::KeyCode::LeftShift) {
            self.radius = 15.0
        }
        if is_key_down(macroquad::input::KeyCode::D) {
            self.x += current_speed;
        }
        if is_key_down(macroquad::input::KeyCode::A) {
            self.x -= current_speed;
        }
        if is_key_down(macroquad::input::KeyCode::S) {
            self.y += current_speed;
        }
        if is_key_down(macroquad::input::KeyCode::W) {
            self.y -= current_speed;
        }
    }
}
struct Bullet {
    x: f32,
    y: f32,
    speed_y: f32,
}
struct EnemyBullet {
    x: f32,
    y: f32,
    speed_y: f32,
    speed_x: f32,
}
struct Enemy {
    x: f32,
    y: f32,
    radius: f32,
}
fn window_conf() -> Conf {
    Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "My Game".to_owned(),
            window_width: 600,
            window_height: 800,
            window_resizable: false,
            ..Default::default()
        },
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut my_ball = Ball {
        x: 400.0,
        y: 500.0,
        speed_x: 4.0,
        speed_y: 3.0,
        radius: 15.0,
        lives: 3,
    };
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemy = Enemy {
        x: 320.0,
        y: 100.0,
        radius: 30.0,
    };
    let mut enemy_bullets: Vec<EnemyBullet> = Vec::new();
    let mut enemy_timer = 0;
    let background_music = load_sound("assets/ready.wav").await.unwrap();
    next_frame().await;
    play_sound(
        &background_music,
        PlaySoundParams {
            looped: true,
            volume: 0.8,
        },
    );

    loop {
        enemy_timer += 1;

        if enemy_timer == 30 {
            let new_enemy_bullet = EnemyBullet {
                x: enemy.x,
                y: enemy.y,
                speed_y: rand::gen_range(2.0, 6.0),
                speed_x: rand::gen_range(-4.0, 4.0),
            };
            enemy_bullets.push(EnemyBullet {
                x: enemy.x,
                y: enemy.y,
                speed_x: -3.0,
                speed_y: 5.0,
            });

            enemy_bullets.push(EnemyBullet {
                x: enemy.x,
                y: enemy.y,
                speed_x: 0.0,
                speed_y: 5.0,
            });

            enemy_bullets.push(EnemyBullet {
                x: enemy.x,
                y: enemy.y,
                speed_x: 3.0,
                speed_y: 5.0,
            });

            enemy_bullets.push(EnemyBullet {
                x: enemy.x,
                y: enemy.y,
                speed_x: -5.0,
                speed_y: 2.0,
            });

            enemy_bullets.push(EnemyBullet {
                x: enemy.x,
                y: enemy.y,
                speed_x: 5.0,
                speed_y: 2.0,
            });
            enemy_bullets.push(EnemyBullet {
                x: enemy.x,
                y: enemy.y,
                speed_x: 2.0,
                speed_y: 2.0,
            });
            enemy_bullets.push(EnemyBullet {
                x: enemy.x,
                y: enemy.y,
                speed_x: 7.0,
                speed_y: 7.0,
            });

            enemy_timer = 0;
        }
        clear_background(DARKBLUE);
        my_ball.check_walls();
        if is_key_pressed(KeyCode::Z) {
            let new_bullet = Bullet {
                x: my_ball.x,
                y: my_ball.y,
                speed_y: -10.0,
            };
            bullets.push(new_bullet);
        }
        for bullet in &mut bullets {
            bullet.y += bullet.speed_y;

            draw_circle(bullet.x, bullet.y, 5.0, WHITE);
        }
        for enemy_bullet in &mut enemy_bullets {
            enemy_bullet.y += enemy_bullet.speed_y;

            enemy_bullet.x += enemy_bullet.speed_x;
            draw_circle(enemy_bullet.x, enemy_bullet.y, 6.0, GREEN);
            let dx = (enemy_bullet.x - my_ball.x).abs();
            let dy = (enemy_bullet.y - my_ball.y).abs();
            if dx < 15.0 && dy < 15.0 {
                my_ball.lives -= 1;
                enemy_bullet.y = 9999.0;
            }
        }
        bullets.retain(|bullet| {
            let dx = bullet.x - enemy.x;
            let dy = bullet.y - enemy.y;
            let distance_sq = (dx * dx) + (dy * dy);

            let radii_sum = 5.0 + enemy.radius;
            let radii_sum_sq = radii_sum * radii_sum;

            let hit_enemy = distance_sq < radii_sum_sq;
            bullet.y > -50.0 && !hit_enemy
        });
        my_ball.handle_input();
        draw_circle(my_ball.x, my_ball.y, my_ball.radius, YELLOW);
        let lives_text = format!("HP: {}", my_ball.lives);
        draw_text(&lives_text, 20.0, 40.0, 30.0, WHITE);
        if my_ball.lives == 0 {
            draw_text("GAME OVER", 120.0, 400.0, 100.0, RED);
            draw_text("Press R to restart", 160.0, 460.0, 30.0, WHITE);
            if is_key_down(input::KeyCode::R) {
                my_ball.lives = 3;
                my_ball.x = 300.0;
                my_ball.y = 600.0;
                enemy_bullets.clear();
                bullets.clear();
            }
        }
        draw_circle(enemy.x, enemy.y, enemy.radius, RED);
        next_frame().await
    }
}
