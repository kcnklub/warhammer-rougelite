use rand::prelude::*;
use raylib::prelude::*;

use crate::utils::{Direction, Position};

pub struct AllEnemies<'a> {
    pub enemies: Vec<Enemy<'a>>,
    time_since_spawn: f32,
    spawn_rate: f32,
    texture: &'a Texture2D,
}

impl<'a> AllEnemies<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        Self {
            enemies: vec![Enemy::new(
                30,
                Position {
                    x: 200.0,
                    y: 200.0,
                    direction: Direction::Down,
                },
                texture,
            )],
            time_since_spawn: 0.0,
            spawn_rate: 3.0,
            texture,
        }
    }

    pub fn tick(&mut self, player_position: &Position, delta: &f32) {
        // retain all alive enemies
        self.enemies.retain(|enemy| enemy.health > 0);

        let speed = 1.0;
        for enemy in self.enemies.iter_mut() {
            let x_delta = (speed * delta) * ((player_position.x - enemy.position.x) / 2.0);
            enemy.position.x += x_delta;
            enemy.position.direction = if x_delta > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            };
            enemy.position.y += (speed * delta) * ((player_position.y - enemy.position.y) / 2.0);
        }
    }

    pub fn spawn_enemies(&mut self, delta: &f32) {
        self.time_since_spawn += delta;
        let mut rng = rand::rng();
        let x: i32 = rng.random_range(1..2480);
        let y: i32 = rng.random_range(1..200);
        if self.time_since_spawn >= self.spawn_rate {
            let spawned_enemy = Enemy::new(
                30,
                Position {
                    x: x as f32,
                    y: y as f32,
                    direction: Direction::Down,
                },
                self.texture,
            );
            self.enemies.push(spawned_enemy);
            self.time_since_spawn = 0.0;
        }
    }
}

pub struct Enemy<'a> {
    pub health: i32,
    pub max_health: i32,

    pub damage: i32,
    pub time_since_last_attack: f32,
    pub attack_speed: f32,

    pub position: Position,
    pub texture: &'a Texture2D,
}

impl<'a> Enemy<'a> {
    pub fn new(health: i32, position: Position, texture: &'a Texture2D) -> Self {
        Self {
            health,
            max_health: health,
            damage: 10,
            time_since_last_attack: 0.0,
            attack_speed: 1.0,
            position,
            texture,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}
