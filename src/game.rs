/*
By: <Mujibullah Muhebullah>
Date: 2026-04-08
Screen Details: The actual game of Space Invaders, where you as a player should shoot and kill the enemies
 */

use crate::modules::bullets::Bullet;
use crate::modules::enemy::Enemy;
use crate::modules::grid::draw_grid;
use crate::modules::player::Player;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub async fn run() -> String {
    let hearts = 3;
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
    let mut player = Player::new("assets/player.png".to_string(), 360.0, 50.0, 700.0, 80.0, 200.0).await;
    let mut enemy = Enemy::new("assets/enemy.png".to_string(), 300.0, 200.0, 60.0, 100.0, 100.0).await;
    use_virtual_resolution(1440.0, 1080.0);
    let mut bullet_list: Vec<crate::modules::bullets::Bullet> = vec![];
    let mut e_bullets: Vec<Bullet> = vec![];
    loop {
        bullet_list.retain(|b| b.get_y() > 0.0);
        clear_background(BLUE);
        img_sidewall1.draw();
        img_sidewall2.draw();
        wall1.draw();
        wall2.draw();
        wall3.draw();
        wall4.draw();
        player.draw();
        enemy.draw();
        enemy.move_enemy(&img_sidewall1, &img_sidewall2);
        player.key_press(&img_sidewall1, &img_sidewall2, &mut bullet_list).await;

        for i in 0..bullet_list.len() {
            bullet_list[i].update();
            bullet_list[i].draw();
            if bullet_list[i].get_y() > 1200.0 {
               bullet_list.remove(i);
                 break;
            }
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
