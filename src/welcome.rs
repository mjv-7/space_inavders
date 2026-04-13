/*
By: <Mujibullah Muhebullah>
Date: 2026-04-08
Screen Details:
 */

use macroquad::prelude::*;
use crate::modules::label::Label;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::grid::{draw_grid};
pub async fn run() -> String{
        let lbl_out = Label::new("Hello\nWorld", 50.0, 100.0, 30);
    loop {
        use_virtual_resolution(1440.0, 1080.0);
        clear_background(RED);
        lbl_out.draw();
        draw_grid(50.0, BLACK);
        next_frame().await;
    }
}