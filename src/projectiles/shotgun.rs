use crate::{enemy::AllEnemies, utils::Position};

use raylib::prelude::*;

#[derive(Clone, Copy)]
pub struct ShotgunProjectile {
    pub speed: f32,
    pub damage: i32,
    pub hits: i32,
    pub position: Position,
    pub angle: f32,
    pub width: f32,
    pub height: f32,
    pub tail_length: f32,
}

impl ShotgunProjectile {
    pub fn new(position: Position, angle: f32) -> Self {
        ShotgunProjectile {
            speed: 900.0,
            damage: 10,
            hits: 0,
            position,
            angle,
            width: 12.0,
            height: 6.0,
            tail_length: 14.0,
        }
    }

    pub fn handle_move(&mut self, delta: &f32) {
        self.position.x += self.angle.cos() * self.speed * delta;
        self.position.y += self.angle.sin() * self.speed * delta;
    }

    pub fn handle_collision(&mut self, all_enemies: &mut AllEnemies) {
        for enemy in all_enemies.enemies.iter_mut() {
            let texture = all_enemies
                .texture_map
                .get(&enemy.enemy_type)
                .expect("unable to find texture");
            let half_width = texture.width as f32 / 2.0;
            let half_height = texture.height as f32 / 2.0;
            let enemy_rec = Rectangle {
                x: enemy.position.x - half_width,
                y: enemy.position.y - half_height,
                width: texture.width as f32,
                height: texture.height as f32,
            };

            let projectile_rect = Rectangle {
                x: self.position.x - self.width / 2.0,
                y: self.position.y - self.height / 2.0,
                width: self.width,
                height: self.height,
            };

            if enemy_rec.check_collision_recs(&projectile_rect) {
                enemy.health -= self.damage;
                println!("Enemy Health: {}", enemy.health);
                self.hits += 1;
            }
        }
    }
}
