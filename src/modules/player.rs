/*
By: Mujibullah
Date: 2026-02-11
Module Details: Player module for handling player movement and collision
 */
use crate::modules::bullets::Bullet;
use crate::modules::collision::check_collision;
use crate::modules::still_image::StillImage;
use crate::modules::{bullets, player};
use ::macroquad::prelude::*;

pub struct Player {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
    old_pos: Vec2,
}

impl Player {
    pub async fn new(asset_path: String, move_speed: f32, x: f32, y: f32, width: f32, height: f32) -> Self {
        Player {
            view: StillImage::new(&asset_path, width, height, x, y, true, 1.0).await,
            move_speed: move_speed,
            movement: vec2(0.0, 0.0),
            old_pos: vec2(0.0, 0.0),
        }
    }
    pub async fn key_press(&mut self, img_sidewall1: &StillImage, img_sidewall2: &StillImage, bullet_list: &mut Vec<Bullet>) -> Vec<Bullet> {
        let mut move_dir = vec2(0.0, 0.0);
        // Keyboard input
        if is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
        }
        if is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::Space) {
            let bullet = bullets::Bullet::new(
                "assets/bullet.png".to_string(),
                self.view.get_x(),
                self.view.get_y(),
                32.0,
                32.0,
                self.movement,
            )
            .await;
            bullet_list.push(bullet);
        }

        // Normalize the movement to prevent faster diagonal movement
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }
        // Apply movement based on frame time
        self.movement = move_dir * self.move_speed * get_frame_time();
        self.old_pos = self.position();

        self.set_x(self.view.get_x() + self.movement.x);
        if check_collision(self.view_player(), img_sidewall1, 1) || check_collision(self.view_player(), img_sidewall2, 1) {
            self.set_x(self.get_x() - self.movement.x);
        }
        bullet_list.to_vec()
    }
    
    pub fn position(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }
    pub fn get_x(&self) -> f32 {
        self.view.get_x()
    }
    pub fn get_y(&self) -> f32 {
        self.view.get_y()
    }
    pub fn set_x(&mut self, x: f32) {
        self.view.set_x(x);
    }
    pub fn set_y(&mut self, y: f32) {
        self.view.set_y(y);
    }
    #[allow(unused)]

    pub fn view_player(&self) -> &StillImage {
        &self.view
    }
    #[allow(unused)]
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.set_x(x);
        self.set_y(y);
    }
    pub fn draw(&mut self) {
        self.view.draw();
    }
    #[allow(unused)]
    pub fn movement(&mut self) -> Vec2 {
        self.movement
    }
    #[allow(unused)]
    pub fn back_x(&mut self) {
        self.set_x(self.old_pos.x);
    }
    #[allow(unused)]
    pub fn back_y(&mut self) {
        self.set_y(self.old_pos.y);
    }
    #[allow(unused)]
    pub fn collision_x(&mut self, img_out: &StillImage) -> bool {
        {
            let mut collision = false;
            if self.movement.x != 0.0 {
                self.set_x(self.view.pos().x + self.movement.x);
                if check_collision(img_out, &self.view, 1) {
                    collision = true;
                    //self.set_x(self.old_pos.x); // Undo if collision happens
                    //println!("Collision detected on X axis!");
                }
            }
            collision
        }
    }
    #[allow(unused)]
    pub fn collision_y(&mut self, img_out: &StillImage) -> bool {
        let mut collision = false;
        if self.movement.y != 0.0 {
            self.set_y(self.get_y() + self.movement.y);
            if check_collision(img_out, &self.view, 1) {
                collision = true;
            }
        }

        collision
    }
    #[allow(unused)]
    pub fn collision(&mut self, img_out: &StillImage) -> bool {
        if check_collision(&self.view, img_out, 1) { true } else { false } // check_collision(obj1, obj2, skip_pixels)
    }
    #[allow(unused)]
    pub async fn set_texture(&mut self, texture_path: &str) {
        self.view.set_texture(texture_path).await;
    }
}
