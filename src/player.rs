use crate::{
    enemy::AllEnemies,
    projectile::{BolterProjectile, Projectile},
    statuses::*,
    weapons::{BolterData, Weapon},
};

use crate::utils::{Direction, Position};
use raylib::ffi::KeyboardKey;
use raylib::prelude::*;

pub struct Player {
    pub position: Position,
    pub move_speed: f32,
    pub health: i32,
    pub max_health: i32,

    // game mechanic data
    pub statuses: Vec<Status>,
    pub weapons: Vec<Weapon>,

    // Rendering bits
    pub texture: Texture2D,
}

impl<'a> Player {
    pub fn new(position: Position, texture: Texture2D) -> Self {
        let mut player = Player {
            position,
            move_speed: 300.0,
            health: 100,
            max_health: 100,
            statuses: vec![],
            weapons: vec![],
            texture: texture,
        };

        player.add_weapon(Weapon::Bolter(BolterData {
            damage: 1.0,
            tick_interval: 1.0,
            time_since_last_tick: 0.0,
        }));

        player
    }

    pub fn handle_user_input(&mut self, rl: &raylib::RaylibHandle, delta: &f32) {
        let speed_multiplier = self.calculate_speed_multiplier();
        let effective_speed = self.move_speed * speed_multiplier;

        // Handle WASD input
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.position.y -= effective_speed * delta;
            self.position.direction = Direction::Up;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.position.y += effective_speed * delta;
            self.position.direction = Direction::Down;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.position.x -= effective_speed * delta;
            self.position.direction = Direction::Left;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.position.x += effective_speed * delta;
            self.position.direction = Direction::Right;
        }
    }

    pub fn handle_status_effects(&mut self, delta: &f32) {
        // Process each status effect
        for status in self.statuses.iter_mut() {
            match status {
                Status::Poison(data) => {
                    data.time_since_last_tick += delta;
                    data.remaining_duration -= delta;

                    if data.time_since_last_tick >= data.tick_interval {
                        self.health -= data.damage_per_tick;
                        data.time_since_last_tick = 0.0;
                    }
                }
                Status::Burn(data) => {
                    data.time_since_last_tick += delta;
                    data.remaining_duration -= delta;

                    if data.time_since_last_tick >= data.tick_interval {
                        self.health -= data.damage_per_tick;
                        data.time_since_last_tick = 0.0;
                    }
                }
                Status::Slow(data) => {
                    data.remaining_duration -= delta;
                }
                Status::Stun(data) => {
                    data.remaining_duration -= delta;
                }
                Status::Regeneration(data) => {
                    data.time_since_last_tick += delta;
                    data.remaining_duration -= delta;

                    if data.time_since_last_tick >= data.tick_interval {
                        self.health += data.heal_per_tick;
                        data.time_since_last_tick = 0.0;
                    }
                }
                Status::SpeedBoost(data) => {
                    data.remaining_duration -= delta;
                }
            }
        }

        // Clamp health between 0 and max_health
        self.health = self.health.max(0).min(self.max_health);

        // Remove expired statuses
        self.statuses.retain(|status| !status.is_expired());
    }

    pub fn handle_weapons(&mut self, delta: &f32) -> Vec<Projectile> {
        let mut res = vec![];
        for weapon in self.weapons.iter_mut() {
            match weapon {
                Weapon::Bolter(data) => {
                    data.time_since_last_tick += delta;

                    if data.time_since_last_tick >= data.tick_interval {
                        println!("Firing the bolter");
                        res.push(Projectile::Bolter(BolterProjectile::new(
                            self.position.clone(),
                        )));
                        data.time_since_last_tick = 0.0;
                    }
                }
            }
        }
        res
    }

    fn calculate_speed_multiplier(&self) -> f32 {
        let mut multiplier = 1.0;

        for status in &self.statuses {
            match status {
                Status::Stun(_) => return 0.0, // Stun overrides everything
                Status::Slow(data) => multiplier *= data.speed_multiplier,
                Status::SpeedBoost(data) => multiplier *= data.speed_multiplier,
                _ => {}
            }
        }

        multiplier
    }

    pub fn add_status(&mut self, status: Status) {
        // Remove existing status of the same type (single instance rule)
        self.statuses.retain(|s| {
            !matches!(
                (s, &status),
                (Status::Poison(_), Status::Poison(_))
                    | (Status::Burn(_), Status::Burn(_))
                    | (Status::Slow(_), Status::Slow(_))
                    | (Status::Stun(_), Status::Stun(_))
                    | (Status::Regeneration(_), Status::Regeneration(_))
                    | (Status::SpeedBoost(_), Status::SpeedBoost(_))
            )
        });

        self.statuses.push(status);
    }

    pub fn get_active_status_names(&self) -> Vec<(String, f32)> {
        self.statuses
            .iter()
            .map(|s| (s.get_display_name().to_string(), s.get_remaining_duration()))
            .collect()
    }

    pub fn add_weapon(&mut self, weapon: Weapon) {
        // Remove existing status of the same type (single instance rule)
        self.weapons
            .retain(|s| !matches!((s, &weapon), (Weapon::Bolter(_), Weapon::Bolter(_))));

        self.weapons.push(weapon);
    }

    pub fn handle_enemies(&mut self, enemies: &mut AllEnemies<'a>, delta: &f32) {
        for enemy in enemies.enemies.iter_mut() {
            let enemy_rec = Rectangle {
                x: enemy.position.x,
                y: enemy.position.y,
                width: enemy.texture.width as f32,
                height: enemy.texture.height as f32,
            };

            let player_point = Vector2 {
                x: self.position.x,
                y: self.position.y,
            };

            enemy.time_since_last_attack += delta;
            if enemy_rec.check_collision_circle_rec(player_point, (self.texture.width / 2) as f32) {
                if enemy.time_since_last_attack >= enemy.attack_speed {
                    self.health -= enemy.damage;
                    enemy.time_since_last_attack = 0.0
                }
            }
        }
    }
}
