/*
By: <Mujibullah Muhebullah>
Date: 2026-04-08
Screen Details: The actual game of Space Invaders, where you as a player should shoot and kill the enemies
 */

use crate::modules::bullets::Bullet;
use crate::modules::collision::check_collision;
use crate::modules::enemy::Enemy;
use crate::modules::grid::draw_grid;
use crate::modules::player::Player;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub async fn run() -> String {
    let mut hearts = 3;
    let heart = StillImage::new(
        "assets/heart.png",
        50.0, // width
        50.0, // height
        0.0,  // x (will be set in the loop)
        20.0, // y position
        true,
        1.0,
    )
    .await;
    let img_sidewall1 = StillImage::new(
        "assets/blackscreen.png",
        10.0,   // width
        1800.0, // height
        -5.0,   // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    let img_sidewall2 = StillImage::new(
        "assets/blackscreen.png",
        10.0,   // width
        1800.0, // height
        1435.0, // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    let wall1 = StillImage::new(
        "assets/blackscreen.png",
        150.0, // width
        50.0,  // height
        50.0,  // x position
        500.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    let wall2 = StillImage::new(
        "assets/blackscreen.png",
        150.0, // width
        50.0,  // height
        360.0, // x position
        500.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    let wall3 = StillImage::new(
        "assets/blackscreen.png",
        150.0, // width
        50.0,  // height
        720.0, // x position
        500.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    let wall4 = StillImage::new(
        "assets/blackscreen.png",
        150.0,  // width
        50.0,   // height
        1200.0, // x position
        500.0,  // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
let img_bg = StillImage::new(
    "assets/cosmos.png",
    1440.0, // width
    1080.0, // height
    0.0,    // x position
    0.0,    // y position
    true,   // Enable stretching
    1.0,    // Normal zoom (100%)
).await;
    let mut enemy_x = 80.0;
    let mut enemy_y = 50.0;
    let mut enemies: Vec<Enemy> = vec![];
    for i in 0..25 {
        let enemy = Enemy::new("assets/enemy.png".to_string(), 300.0, enemy_x, enemy_y, 100.0, 100.0).await;
        enemy_x += 90.0; // Move the next enemy to the right
        if enemy_x > 800.0 {
            // If we reach the edge of the screen
            enemy_x = 120.0; // Reset x position
            enemy_y += 90.0; // Move the next row down
        }
        enemies.push(enemy);
    }
    let mut player = Player::new("assets/player.png".to_string(), 360.0, 50.0, 700.0, 80.0, 200.0).await;
    use_virtual_resolution(1440.0, 1080.0);
    let enemy_bullet_template = Bullet::new("assets/bullet.png".to_string(), 0.0, 0.0, 20.0, 40.0, Vec2::new(0.0, 1.0)).await;
    let bullet_template = Bullet::new("assets/bullet.png".to_string(), 0.0, 0.0, 20.0, 40.0, Vec2::new(0.0, -1.0)).await;

    let mut bullet_list: Vec<Bullet> = vec![];
    let mut e_bullets: Vec<Bullet> = vec![];
    let mut bullet_cooldown = get_time();
    loop {
        bullet_list.retain(|b| b.get_y() > 0.0);
        clear_background(BLUE);
        for enemy in 0..enemies.len() {
            enemies[enemy].move_enemy(&img_sidewall1, &img_sidewall2);
        }
        img_bg.draw();
        img_sidewall1.draw();
        img_sidewall2.draw();
        wall1.draw();
        wall2.draw();
        wall3.draw();
        wall4.draw();
        player.draw();
        for enemy in 0..enemies.len() {
            enemies[enemy].draw();
        }

        player.key_press(&img_sidewall1, &img_sidewall2, &mut bullet_list, &bullet_template).await;
        if get_time() - bullet_cooldown > 0.67 {
            if !enemies.is_empty() {
                let idx = macroquad::rand::gen_range(0, enemies.len());
                let spawn = &enemies[idx];
                let bullet_x = spawn.get_x();
                let bullet_y = spawn.get_y();

                let mut bullet = enemy_bullet_template.clone();
                bullet.set_position(bullet_x, bullet_y);
                e_bullets.push(bullet);
            }
            bullet_cooldown = get_time();
        }

        'bullet_loop: for i in 0..bullet_list.len() {
            bullet_list[i].update();
            bullet_list[i].draw();

            for j in 0..enemies.len() {
                if check_collision(bullet_list[i].view(), enemies[j].view(), 1) {
                    enemies.remove(j);
                    bullet_list.remove(i);
                    break 'bullet_loop;
                }
            }
            if check_collision(bullet_list[i].view(), &wall1, 1)
                || check_collision(bullet_list[i].view(), &wall2, 1)
                || check_collision(bullet_list[i].view(), &wall3, 1)
                || check_collision(bullet_list[i].view(), &wall4, 1)
            {
                bullet_list.remove(i);
                break;
            }

            if bullet_list[i].get_y() < 0.0 {
                bullet_list.remove(i);
                break;
            }
        }

               'enemy_bullet_loop: for i in 0..e_bullets.len() {
            e_bullets[i].update();
            e_bullets[i].draw();

            if check_collision(e_bullets[i].view(), player.view_player(), 1) {
                e_bullets.remove(i);
                hearts -= 1;
                break 'enemy_bullet_loop;
            }

            let by = e_bullets[i].get_y();
            let bx = e_bullets[i].get_x();

            let hits_wall =
                (bx >= 50.0   && bx <= 200.0  && by >= 500.0 && by <= 550.0) ||
                (bx >= 360.0  && bx <= 510.0  && by >= 500.0 && by <= 550.0) ||
                (bx >= 720.0  && bx <= 870.0  && by >= 500.0 && by <= 550.0) ||
                (bx >= 1200.0 && bx <= 1350.0 && by >= 500.0 && by <= 550.0);

            if hits_wall {
                e_bullets.remove(i);
                break 'enemy_bullet_loop;
            }

            if e_bullets[i].get_y() > 1020.0 {
                e_bullets.remove(i);
                break 'enemy_bullet_loop;
            }
        }
        if enemies.is_empty() || hearts < 1 {
            return "welcome".to_string();
        }
        for i in 0..hearts {
            let x = 20.0 + (i as f32) * 60.0;
            let mut h = heart.clone();
            h.set_x(x);
            h.draw();
        }
        draw_grid(40.0, BLACK);
        next_frame().await;
    }
}