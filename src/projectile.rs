use std::vec;

use raylib::prelude::*;

use crate::{
    enemy::AllEnemies,
    player::Player,
    utils::{Direction, Position},
};

const SCREEN_HALF_WIDTH: f32 = 1240.0;
const SCREEN_HALF_HEIGHT: f32 = 720.0;
const CULL_BUFFER: f32 = 200.0; // Extra margin before removing

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

    pub fn move_projectiles(&mut self, player: &Player, delta: &f32) {
        for projectile in self.projectiles.iter_mut() {
            match projectile {
                Projectile::Bolter(bolter_data) => {
                    let angle = bolter_data.angle;
                    bolter_data.position.x += angle.cos() * bolter_data.speed * delta;
                    bolter_data.position.y += angle.sin() * bolter_data.speed * delta;
                }
                Projectile::PowerSword(sword_data) => {
                    sword_data.lifetime -= delta;
                }
            };
        }

        // Remove projectiles that have left the visible area
        let cull_left = player.position.x - SCREEN_HALF_WIDTH - CULL_BUFFER;
        let cull_right = player.position.x + SCREEN_HALF_WIDTH + CULL_BUFFER;
        let cull_top = player.position.y - SCREEN_HALF_HEIGHT - CULL_BUFFER;
        let cull_bottom = player.position.y + SCREEN_HALF_HEIGHT + CULL_BUFFER;

        self.projectiles.retain(|projectile| {
            let pos = match projectile {
                Projectile::Bolter(b) => &b.position,
                Projectile::PowerSword(s) => &s.position,
            };
            pos.x >= cull_left && pos.x <= cull_right &&
            pos.y >= cull_top && pos.y <= cull_bottom
        });
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
                Projectile::PowerSword(_) => {}
            };
        }

        self.projectiles.retain(|&projectile| match projectile {
            Projectile::Bolter(bolter_projectile) => bolter_projectile.hits == 0,
            Projectile::PowerSword(sword_projectile) => sword_projectile.lifetime > 0.0,
        });
    }
}

#[derive(Clone, Copy)]
pub enum Projectile {
    Bolter(BolterProjectile),
    PowerSword(PowerSwordProjectile),
}

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
}

#[derive(Clone, Copy)]
pub struct PowerSwordProjectile {
    pub damage: i32,
    pub hits: i32,
    pub position: Position,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub width: f32,
    pub height: f32,
    pub slash_distance: f32,
}

impl PowerSwordProjectile {
    pub fn new(position: Position) -> Self {
        PowerSwordProjectile {
            damage: 25,
            hits: 0,
            position,
            lifetime: 0.25,
            max_lifetime: 0.25,
            width: 80.0,
            height: 20.0,
            slash_distance: 60.0,
        }
    }

    pub fn get_slash_progress(&self) -> f32 {
        1.0 - (self.lifetime / self.max_lifetime)
    }

    pub fn get_slash_offset(&self) -> f32 {
        let progress = self.get_slash_progress();
        (progress - 0.5) * self.slash_distance
    }

    pub fn get_collision_rect(&self) -> Rectangle {
        let slash_offset = self.get_slash_offset();
        match self.position.direction {
            Direction::Up => Rectangle {
                x: self.position.x - self.height / 2.0 + slash_offset,
                y: self.position.y - self.width,
                width: self.height,
                height: self.width,
            },
            Direction::Down => Rectangle {
                x: self.position.x - self.height / 2.0 - slash_offset,
                y: self.position.y,
                width: self.height,
                height: self.width,
            },
            Direction::Left => Rectangle {
                x: self.position.x - self.width,
                y: self.position.y - self.height / 2.0 - slash_offset,
                width: self.width,
                height: self.height,
            },
            Direction::Right => Rectangle {
                x: self.position.x,
                y: self.position.y - self.height / 2.0 + slash_offset,
                width: self.width,
                height: self.height,
            },
        }
    }
}
