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
        window_width: 1024,
        window_height: 768,
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
    let mut last_switch = get_time() - 0.02;
    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "welcome" => welcome::run().await,
                _ => break,
            };
            last_switch = get_time();
        }
        clear_background(RED);

        
        next_frame().await;
    }
}
