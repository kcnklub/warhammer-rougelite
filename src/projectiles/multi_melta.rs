use crate::{enemy::AllEnemies, utils::Position};

use raylib::prelude::*;

#[derive(Clone, Copy)]
pub struct MultiMeltaProjectile {
    pub speed: f32,
    pub damage: i32,
    pub position: Position,
    pub angle: f32,
    pub distance_traveled: f32,
    pub max_range: f32,
    pub width_start: f32,
    pub width_end: f32,
    pub length: f32,
}

impl MultiMeltaProjectile {
    pub fn new(position: Position, angle: f32) -> Self {
        MultiMeltaProjectile {
            speed: 1000.0,
            damage: 2,
            position,
            angle,
            distance_traveled: 0.0,
            max_range: 350.0,
            width_start: 28.0,
            width_end: 140.0,
            length: 50.0,
        }
    }

    pub fn handle_move(&mut self, delta: &f32) {
        let step = self.speed * delta;
        self.position.x += self.angle.cos() * step;
        self.position.y += self.angle.sin() * step;
        self.distance_traveled += step;
    }

    pub fn current_width(&self) -> f32 {
        let t = (self.distance_traveled / self.max_range).clamp(0.0, 1.0);
        self.width_start + (self.width_end - self.width_start) * t
    }

    fn collision_centers(&self) -> [Vector2; 3] {
        let forward = Vector2::new(self.angle.cos(), self.angle.sin());
        let half = self.length * 0.5;
        let center = Vector2::new(self.position.x, self.position.y);

        [
            Vector2::new(center.x - forward.x * half, center.y - forward.y * half),
            center,
            Vector2::new(center.x + forward.x * half, center.y + forward.y * half),
        ]
    }

    pub fn handle_collision(&mut self, all_enemies: &mut AllEnemies) {
        let radius = self.current_width() * 0.5;
        let centers = self.collision_centers();

        for enemy in all_enemies.enemies.iter_mut() {
            let texture = all_enemies
                .texture_map
                .get(&enemy.enemy_type)
                .expect("unable to find texture");
            let half_width = texture.width as f32 / 2.0;
            let half_height = texture.height as f32 / 2.0;
            let enemy_rec = Rectangle::new(
                enemy.position.x - half_width,
                enemy.position.y - half_height,
                texture.width as f32,
                texture.height as f32,
            );

            let mut collided = false;
            for center in centers {
                if enemy_rec.check_collision_circle_rec(center, radius) {
                    collided = true;
                    break;
                }
            }

            if collided {
                enemy.health -= self.damage;
            }
        }
    }
}
