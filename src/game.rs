/*
By: <Mujibullah Muhebullah>
Date: 2026-04-08
Screen Details: The actual game of Space Invaders, where you as a player should shoot and kill the enemies
 */

use macroquad::prelude::*;
use crate::modules::label::Label;
use crate::modules::still_image::StillImage;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::grid::{draw_grid};

pub async fn run() -> String{ 
    let img_player = StillImage::new(
        "assets/spaceship1.png",
        100.0,  // width
        200.0,  // height
        200.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    use_virtual_resolution(1440.0, 1080.0);
    let lbl_out = Label::new("Hello\nWorld", 50.0, 100.0, 30);
    loop {
        clear_background(WHITE);
        lbl_out.draw();
        img_player.draw();
        draw_grid(50.0, BLACK);      
        next_frame().await;
    }
}

