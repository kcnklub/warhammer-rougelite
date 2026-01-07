use std::collections::HashMap;

use rand::prelude::*;
use raylib::prelude::*;

use crate::utils::{Direction, Position};

pub struct AllEnemies<'a> {
    pub enemies: Vec<Enemy>,
    time_since_spawn: f32,
    spawn_rate: f32,
    pub texture_map: HashMap<EnemyType, &'a Texture2D>,
}

impl<'a> AllEnemies<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        let mut texture_map = HashMap::new();
        texture_map.insert(EnemyType::servo_skull_type(), texture);

        Self {
            enemies: vec![],
            time_since_spawn: 0.0,
            spawn_rate: 3.0,
            texture_map,
        }
    }

    pub fn tick(&mut self, player_position: &Position, delta: &f32) {
        // retain all alive enemies
        self.enemies.retain(|enemy| enemy.health > 0);

        let speed = 300.0;
        for enemy in self.enemies.iter_mut() {
            // Calculate direction vector
            let dx = player_position.x - enemy.position.x;
            let dy = player_position.y - enemy.position.y;

            // Calculate distance (magnitude of direction vector)
            let distance = (dx * dx + dy * dy).sqrt();

            // Normalize and apply speed (only if distance > 0 to avoid division by zero)
            if distance > 0.0 {
                let x_delta = (dx / distance) * speed * delta;
                let y_delta = (dy / distance) * speed * delta;

                enemy.position.x += x_delta;
                enemy.position.y += y_delta;

                enemy.position.direction = if x_delta > 0.0 {
                    Direction::Right
                } else {
                    Direction::Left
                };
            }
        }
    }

    pub fn spawn_enemies(&mut self, delta: &f32) {
        self.time_since_spawn += delta;
        let mut rng = rand::rng();
        let x: i32 = rng.random_range(1..2480);
        let y: i32 = rng.random_range(1..200);
        if self.time_since_spawn >= self.spawn_rate {
            let spawned_enemy = EnemyType::new_servo_skull(Position {
                x: x as f32,
                y: y as f32,
                direction: Direction::Down,
            });
            self.enemies.push(spawned_enemy);
            self.time_since_spawn = 0.0;
        }
    }
}

pub struct Enemy {
    pub enemy_type: EnemyType,

    pub health: i32,
    pub max_health: i32,
    pub speed: i32,

    pub damage: i32,
    pub time_since_last_attack: f32,
    pub attack_speed: f32,

    pub position: Position,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnemyType {
    ServoSkull,
    DarkFighter,
}

impl EnemyType {
    pub fn servo_skull_type() -> EnemyType {
        EnemyType::ServoSkull
    }

    pub fn new_servo_skull(position: Position) -> Enemy {
        Enemy {
            enemy_type: EnemyType::ServoSkull,
            health: 30,
            max_health: 30,
            speed: 450,
            damage: 10,
            time_since_last_attack: 0.0,
            attack_speed: 1.0,
            position,
        }
    }

    pub fn dark_fighter_type() -> EnemyType {
        EnemyType::DarkFighter
    }

    pub fn new_dark_fighter(position: Position) -> Enemy {
        Enemy {
            enemy_type: EnemyType::DarkFighter,
            health: 30,
            max_health: 30,
            speed: 450,
            damage: 10,
            time_since_last_attack: 0.0,
            attack_speed: 1.0,
            position,
        }
    }
}
