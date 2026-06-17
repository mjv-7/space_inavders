//By Mujib  
//June 17th 2026
//Enemy module

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;

pub struct Enemy {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
}

impl Enemy {
    pub async fn new(asset_path: String, move_speed: f32, x: f32, y: f32, width: f32, height: f32) -> Self {
        Enemy {
            view: StillImage::new(&asset_path, width, height, x, y, true, 1.0).await,
            move_speed: 250.0,
            movement: Vec2::new(1.0, 0.0),
        }
    }
    pub fn draw(&mut self) {
        self.view.draw();
    }

    pub fn move_enemy(&mut self, img_sidewall1: &StillImage, img_sidewall2: &StillImage) {
    self.view.set_x(self.view.get_x() + self.movement.x * self.move_speed * get_frame_time());
    if check_collision(&self.view, img_sidewall1, 1) || check_collision(&self.view, img_sidewall2, 1) {
        
        let original_direction = self.movement.x;
        self.movement.x = -self.movement.x; // flip direction
        
        self.view.set_x(self.view.get_x() - original_direction * self.move_speed * get_frame_time());
    }
}
      #[allow(unused)]
    pub fn view_player(&self) -> &StillImage {
        &self.view
    }
    // Setter for position
    #[allow(unused)]
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.view.set_x(x);
        self.view.set_y(y);
        self
    }
    #[allow(unused)]
    pub fn get_x(&self) -> f32 {
        self.view.get_x()
    }

    #[allow(unused)]
    pub fn set_x(&mut self, x: f32) {
        self.view.set_x(x);
    }

    // Get and set y position
    #[allow(unused)]
    pub fn get_y(&self) -> f32 {
        self.view.get_y()
    }

    #[allow(unused)]
    pub fn set_y(&mut self, y: f32) {
        self.view.set_y(y);
    }
    #[allow(unused)]
    pub fn pos(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }
}