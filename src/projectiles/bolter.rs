use crate::{enemy::AllEnemies, utils::Position};

use raylib::prelude::*;

#[derive(Clone, Copy)]
pub struct BolterProjectile {
    pub speed: f32,
    pub damage: i32,
    pub hits: i32,
    pub position: Position,
    pub angle: f32,
}

impl BolterProjectile {
    pub fn new(position: Position, angle: f32) -> Self {
        BolterProjectile {
            speed: 1000.0,
            damage: 10,
            hits: 0,
            position,
            angle,
        }
    }

    pub fn handle_move(&mut self, delta: &f32) {
        let angle = self.angle;
        self.position.x += angle.cos() * self.speed * delta;
        self.position.y += angle.sin() * self.speed * delta;
    }

    pub fn handle_collision(&mut self, all_enemies: &mut AllEnemies, proj_texture: &Texture2D) {
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
                x: self.position.x,
                y: self.position.y,
                width: proj_texture.width as f32,
                height: proj_texture.height as f32,
            };
            if enemy_rec.check_collision_recs(&projectile_rect) {
                enemy.health -= self.damage;
                println!("Enemy Health: {}", enemy.health);
                self.hits += 1;
            }
        }
    }
}
