/*
By: <Mujibullah Muhebullah>
Date: 2026-04-08
Program Details: A space invaders game, consisting of enemy ships and player ship, attempting to destroy the player ship. and the player ship attempting to destroy the enemy ships
*/

mod modules;
mod welcome;
mod game;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "space_inavders".to_string(),
        window_width: 1440,
        window_height: 1080,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_screen = "welcome".to_string();
    loop {
        current_screen = match current_screen.as_str() {
            "welcome" => welcome::run().await,
            "game" => game::run().await,
            _ => break,
        };
    }
}