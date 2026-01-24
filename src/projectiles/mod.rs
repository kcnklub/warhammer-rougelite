use std::vec;

use raylib::prelude::*;

use crate::{enemy::AllEnemies, player::Player};

pub mod bolter;
pub mod power_sword;
pub mod shotgun;

const SCREEN_HALF_WIDTH: f32 = 1240.0;
const SCREEN_HALF_HEIGHT: f32 = 720.0;
const CULL_BUFFER: f32 = 200.0; // Extra margin before removing

#[derive(Clone, Copy)]
pub enum Projectile {
    Bolter(bolter::BolterProjectile),
    PowerSword(power_sword::PowerSwordProjectile),
    Shotgun(shotgun::ShotgunProjectile),
}

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
                Projectile::Bolter(bolter_data) => bolter_data.handle_move(delta),
                Projectile::PowerSword(sword_data) => sword_data.handle_move(player, delta),
                Projectile::Shotgun(shotgun_data) => shotgun_data.handle_move(delta),
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
                Projectile::Shotgun(s) => &s.position,
            };
            pos.x >= cull_left && pos.x <= cull_right && pos.y >= cull_top && pos.y <= cull_bottom
        });
    }

    pub fn handle_collision(&mut self, all_enemies: &mut AllEnemies) {
        for projectile in self.projectiles.iter_mut() {
            match projectile {
                Projectile::Bolter(bolter_data) => {
                    bolter_data.handle_collision(all_enemies, self.texture)
                }
                Projectile::PowerSword(sword_data) => sword_data.handle_collision(all_enemies),
                Projectile::Shotgun(shotgun_data) => shotgun_data.handle_collision(all_enemies),
            };
        }

        self.projectiles.retain(|&projectile| match projectile {
            Projectile::Bolter(bolter_projectile) => bolter_projectile.hits == 0,
            Projectile::PowerSword(sword_projectile) => sword_projectile.lifetime > 0.0,
            Projectile::Shotgun(shotgun_projectile) => shotgun_projectile.hits == 0,
        });
    }
}
