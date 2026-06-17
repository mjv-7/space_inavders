//By Mujib
//June 17th 2026
//Bullet module

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;
#[derive(Clone,)]
pub struct Bullet {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
}

impl Bullet {
    pub async fn new(asset_path: String, x: f32, y: f32, width: f32, height: f32, movement: Vec2) -> Self {
        Bullet {
            view: StillImage::new("assets/bullet.png", width, height, x, y, true, 1.0).await,
            move_speed: 250.0,
            movement: Vec2::new(movement.x,movement.y),
        }
    }

    pub fn draw(&mut self) {
        self.view.draw();
    }

    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.view.set_x(x);
        self.view.set_y(y);
        self
    }
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
    pub fn update(&mut self) {
    let new_y = self.view.get_y() - self.move_speed * get_frame_time();
    self.view.set_y(new_y);
}

}
