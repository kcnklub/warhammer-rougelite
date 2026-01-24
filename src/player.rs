use crate::{
    projectiles::{
        bolter::BolterProjectile, multi_melta::MultiMeltaProjectile,
        power_sword::PowerSwordProjectile, shotgun::ShotgunProjectile, Projectile,
    },
    statuses::*,
    weapons::{Weapon, WeaponData},
};

use crate::utils::{Direction, Position};
use raylib::ffi::KeyboardKey;
use raylib::prelude::*;
use std::f32::consts::PI;

// Scale factor for player sprite rendering (higher value = smaller sprite)
pub const PLAYER_SCALE: f32 = 1.5;

pub struct MouseInformation(f32);

impl MouseInformation {
    pub fn get_angle_degrees(&self) -> f32 {
        self.0.to_degrees()
    }

    pub fn get_direction(&self) -> Direction {
        let abs_value = self.get_angle_degrees().abs();
        if abs_value >= 90.0 {
            Direction::Left
        } else {
            Direction::Right
        }
    }
}

pub struct Player {
    pub position: Position,
    /// derived from the mouse aiming
    pub aiming_direction: Direction,
    /// derived from the player moving.
    pub moving_direction: Direction,

    pub mouse_info: MouseInformation,

    pub move_speed: f32,
    pub health: i32,
    pub max_health: i32,

    // game mechanic data
    pub statuses: Vec<Status>,
    pub weapons: [Option<Weapon>; 3],

    // Rendering bits
    pub texture: Texture2D,
    pub collision_radius: f32,
}

impl<'a> Player {
    pub fn new(position: Position, texture: Texture2D) -> Self {
        // Calculate collision radius based on scaled sprite size
        let collision_radius = (texture.width as f32 / PLAYER_SCALE) / 2.0;

        Player {
            position,
            aiming_direction: Direction::Right,
            moving_direction: Direction::Right,
            mouse_info: MouseInformation(0.0),
            move_speed: 300.0,
            health: 100,
            max_health: 100,
            statuses: vec![],
            weapons: [
                Some(Weapon::Shotgun(WeaponData {
                    damage: 1.0,
                    tick_interval: 0.9,
                    time_since_last_tick: 0.0,
                })),
                Some(Weapon::MultiMelta(WeaponData {
                    damage: 1.0,
                    tick_interval: 1.1,
                    time_since_last_tick: 0.0,
                })),
                None,
            ],
            texture,
            collision_radius,
        }
    }

    pub fn update_aim_direction(
        &mut self,
        rl: &raylib::RaylibHandle,
        camera: raylib::ffi::Camera2D,
    ) {
        let mouse_screen = rl.get_mouse_position();
        let mouse_world = rl.get_screen_to_world2D(mouse_screen, camera);

        let dx = mouse_world.x - self.position.x;
        let dy = mouse_world.y - self.position.y;

        // Calculate angle in radians (atan2 returns -PI to PI)
        let angle = dy.atan2(dx);
        self.mouse_info = MouseInformation(angle);
        self.aiming_direction = self.mouse_info.get_direction();
    }

    pub fn handle_user_input(&mut self, rl: &raylib::RaylibHandle, delta: &f32) {
        let speed_multiplier = self.calculate_speed_multiplier();
        let effective_speed = self.move_speed * speed_multiplier;

        // Handle WASD input (movement only, direction is handled by mouse)
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.moving_direction = Direction::Up;
            self.position.y -= effective_speed * delta;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.moving_direction = Direction::Down;
            self.position.y += effective_speed * delta;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.moving_direction = Direction::Left;
            self.position.x -= effective_speed * delta;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.moving_direction = Direction::Right;
            self.position.x += effective_speed * delta;
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
        for slot in self.weapons.iter_mut() {
            let Some(weapon) = slot else { continue };
            match weapon {
                Weapon::Bolter(data) => {
                    data.time_since_last_tick += delta;

                    let offset = (self.texture.width / 2) as f32;

                    if data.time_since_last_tick >= data.tick_interval {
                        let angle = self.mouse_info.0;
                        let position = Position {
                            x: self.position.x + angle.cos() * offset,
                            y: self.position.y + angle.sin() * offset,
                        };
                        res.push(Projectile::Bolter(BolterProjectile::new(
                            position,
                            self.mouse_info.0,
                        )));
                        data.time_since_last_tick = 0.0;
                    }
                }
                Weapon::PowerSword(data) => {
                    data.time_since_last_tick += delta;
                    let offset = (self.texture.width / 2) as f32;

                    let rotation = match self.moving_direction {
                        Direction::Up => 1.0,
                        Direction::Down => 1.0,
                        Direction::Left => -1.0,
                        Direction::Right => 1.0,
                    };
                    if data.time_since_last_tick >= data.tick_interval {
                        let position = Position {
                            x: self.position.x + (offset * rotation),
                            y: self.position.y,
                        };
                        res.push(Projectile::PowerSword(PowerSwordProjectile::new(
                            position,
                            self.moving_direction,
                        )));
                        data.time_since_last_tick = 0.0;
                    }
                }
                Weapon::Shotgun(data) => {
                    data.time_since_last_tick += delta;
                    let offset = (self.texture.width / 2) as f32;

                    if data.time_since_last_tick >= data.tick_interval {
                        let base_angle = match self.moving_direction {
                            Direction::Up => -PI / 2.0,
                            Direction::Down => PI / 2.0,
                            Direction::Left => PI,
                            Direction::Right => 0.0,
                        };
                        let spread = 10.0_f32.to_radians();
                        let angles = [
                            base_angle,
                            base_angle - spread,
                            base_angle + spread,
                            base_angle - (spread * 2.0),
                            base_angle + (spread * 2.0),
                        ];

                        for angle in angles {
                            let position = Position {
                                x: self.position.x + angle.cos() * offset,
                                y: self.position.y + angle.sin() * offset,
                            };
                            res.push(Projectile::Shotgun(ShotgunProjectile::new(position, angle)));
                        }

                        data.time_since_last_tick = 0.0;
                    }
                }
                Weapon::MultiMelta(data) => {
                    data.time_since_last_tick += delta;
                    let offset = (self.texture.width / 2) as f32;

                    if data.time_since_last_tick >= data.tick_interval {
                        let angle = self.mouse_info.0;
                        let position = Position {
                            x: self.position.x + angle.cos() * offset,
                            y: self.position.y + angle.sin() * offset,
                        };
                        res.push(Projectile::MultiMelta(MultiMeltaProjectile::new(
                            position, angle,
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

    pub fn get_weapon_slots(&self) -> [Option<&str>; 3] {
        [
            self.weapons[0].as_ref().map(|w| w.get_display_name()),
            self.weapons[1].as_ref().map(|w| w.get_display_name()),
            self.weapons[2].as_ref().map(|w| w.get_display_name()),
        ]
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}
