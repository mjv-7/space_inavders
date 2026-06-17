//By Mujib
//June 17th 2026
//Bullet module

use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;

pub struct Bullet {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
}

impl Bullet {
    pub async fn new(view: StillImage, move_speed: f32, movement: Vec2) -> Self {
        let mut view = StillImage::new("assets/bullet.png", 16.0, 16.0, 0.0, 0.0, false, 1.0).await;
        Bullet {
            view,
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
    pub fn bullet_move (){
        
    }

}