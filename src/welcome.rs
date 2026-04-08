/*
By: <Mujibullah Muhebullah>
Date: 2026-04-08
Screen Details:
 */

use macroquad::prelude::*;
use crate::modules::label::Label;

pub async fn run() -> String{
    let lbl_out = Label::new("Hello\nWorld", 50.0, 100.0, 30);
    loop {
        lbl_out.draw();
        next_frame().await;
    }
}