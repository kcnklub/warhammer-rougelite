use std::vec;

use raylib::prelude::*;

use crate::{
    enemy::AllEnemies,
    utils::{Direction, Position},
};

pub struct AllProjectiles {
    pub projectiles: Vec<Projectile>,
}

impl AllProjectiles {
    pub fn new() -> Self {
        AllProjectiles {
            projectiles: vec![],
        }
    }
    pub fn append(&mut self, new: &mut Vec<Projectile>) {
        self.projectiles.append(new);
    }

    pub fn move_projectiles(&mut self, delta: &f32) {
        for projectile in self.projectiles.iter_mut() {
            match projectile {
                Projectile::Bolter(bolter_data) => match bolter_data.position.direction {
                    Direction::Up => bolter_data.position.y -= bolter_data.speed * delta,
                    Direction::Down => bolter_data.position.y += bolter_data.speed * delta,
                    Direction::Left => bolter_data.position.x -= bolter_data.speed * delta,
                    Direction::Right => bolter_data.position.x += bolter_data.speed * delta,
                },
            };
        }
    }

    pub fn handle_collision(&mut self, all_enemies: &mut AllEnemies) {
        for projectile in self.projectiles.iter_mut() {
            match projectile {
                Projectile::Bolter(bolter_data) => {
                    for enemy in all_enemies.enemies.iter_mut() {
                        let enemy_rec = Rectangle {
                            x: enemy.position.x,
                            y: enemy.position.y,
                            width: enemy.texture.width as f32,
                            height: enemy.texture.height as f32,
                        };
                        let proj_point = Vector2 {
                            x: bolter_data.position.x,
                            y: bolter_data.position.y,
                        };

                        if enemy_rec.check_collision_point_rec(proj_point) {
                            enemy.health -= bolter_data.damage;
                        }
                    }
                }
            };
        }

        all_enemies.enemies.retain(|enemy| enemy.health > 0);
    }
}

#[derive(Clone, Copy)]
pub enum Projectile {
    Bolter(BolterProjectile),
}

#[derive(Clone, Copy)]
pub struct BolterProjectile {
    pub speed: f32,
    pub damage: i32,
    pub position: Position,
}

impl BolterProjectile {
    pub fn new(position: Position) -> Self {
        BolterProjectile {
            speed: 600.0,
            damage: 20,
            position,
        }
    }
}
