/*
By: <Mujibullah Muhebullah>
Date: 2026-04-08
Screen Details:
 */

use crate::modules::label::Label;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;
pub async fn run() -> String {
    let mut img_ship = StillImage::new(
        "assets/player.png",
        100.0,  // width
        250.0,  // height
        1070.0, // x (will be set in the loop)
        200.0,  // y position
        true,
        1.0,
    )
    .await;
    let img_enemy = StillImage::new(
        "assets/enemy.png",
        150.0,  // width
        150.0,  // height
        340.0, // x (will be set in the loop)
        540.0,  // y position
        true,
        1.0,
    )
    .await;
    let mut img_bullet = StillImage::new(
        "assets/bullet.png",
        35.0,  // width
        35.0,  // height
        960.0, // x (will be set in the loop)
        380.0,  // y position
        true,
        1.0,
    ).await;
    let mut img_bullet2 = StillImage::new(
        "assets/bullet.png",
        35.0,  // width
        35.0,  // height
        480.0, // x (will be set in the loop)
        520.0,  // y position
        true,
        1.0,
    ).await;
    img_bullet.set_angle(70.0);
    img_bullet2.set_angle(-90.0);
    img_ship.set_angle(-90.0);
    let mut lbl_out = Label::new(
        "Hello To the Space Invaders
Your Objective is to Defeat the Aliens",
        50.0,
        100.0,
        30,
    );
    lbl_out.with_colors(WHITE, Some(DARKGRAY));
    let btn_next = TextButton::new(640.0, 480.0, 200.0, 60.0, "Click to Play!", BLUE, GREEN, 30);
    let mut lbl_directions = Label::new(
        "A and D keys are for movement, And Space to fire
    your main objective is to destroy the Aliens,
    and not loose your three hearts
After loosing or winning you would be sent back to the welcome menu",
        50.0,
        800.0,
        42,
    );
    lbl_directions.with_colors(WHITE, Some(DARKGRAY));
    let img_bg = StillImage::new(
        "assets/bg.png",
        1440.0, // width
        1080.0, // height
        0.0,    // x (will be set in the loop)
        0.0,    // y position
        true,
        1.0,
    )
    .await;
    loop {
        use_virtual_resolution(1440.0, 1080.0);
        clear_background(BLUE);
        
        
        
        img_bg.draw();
        img_ship.draw();
        img_enemy.draw();
        lbl_out.draw();
        lbl_directions.draw();
        img_bullet.draw();
        img_bullet2.draw();
        if btn_next.click() {
            return "game".to_string();
        }
        next_frame().await;
    }
}
