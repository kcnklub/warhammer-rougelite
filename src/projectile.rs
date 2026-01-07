use std::vec;

use raylib::prelude::*;

use crate::{
    enemy::AllEnemies,
    utils::{Direction, Position},
};

pub struct AllProjectiles<'a> {
    pub projectiles: Vec<Projectile>,

    pub texture: &'a Texture2D,
}

impl<'a> AllProjectiles<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        AllProjectiles {
            projectiles: vec![],
            texture,
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
                        let texture = all_enemies
                            .texture_map
                            .get(&enemy.enemy_type)
                            .expect("unable to find texture");
                        let enemy_rec = Rectangle {
                            x: enemy.position.x,
                            y: enemy.position.y,
                            width: texture.width as f32,
                            height: texture.height as f32,
                        };

                        let dest_rec = Rectangle {
                            x: bolter_data.position.x,
                            y: bolter_data.position.y,
                            width: self.texture.width as f32,
                            height: self.texture.height as f32,
                        };
                        if enemy_rec.check_collision_recs(&dest_rec) {
                            enemy.health -= bolter_data.damage;
                            println!("Enemy Health: {}", enemy.health);
                            bolter_data.hits += 1;
                        }
                    }
                }
            };
        }

        self.projectiles.retain(|&projectile| match projectile {
            Projectile::Bolter(bolter_projectile) => bolter_projectile.hits == 0,
        });
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
    pub hits: i32,
    pub position: Position,
}

impl BolterProjectile {
    pub fn new(position: Position) -> Self {
        BolterProjectile {
            speed: 1000.0,
            damage: 10,
            hits: 0,
            position,
        }
    }
}
