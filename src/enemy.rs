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
            spawn_rate: 1.0,
            texture,
        }
    }

    pub fn tick(&mut self, player_position: &Position, delta: &f32) {
        let speed = 1.0;
        for enemy in self.enemies.iter_mut() {
            enemy.position.x += (speed * delta) * (player_position.x - enemy.position.x);
            enemy.position.y += (speed * delta) * (player_position.y - enemy.position.y);
        }
    }

    pub fn follow_player(&mut self, player_position: &Position, delta: &f32) {
        let speed = 1.0;
        for enemy in self.enemies.iter_mut() {
            let player_pos = Vector2 {
                x: player_position.x,
                y: player_position.y,
            };
            let enemy_pos = Vector2 {
                x: enemy.position.x,
                y: enemy.position.y,
            };
            let distance = enemy_pos.distance_to(player_pos);
            //let speed = speed * distance.log(100.0);
            if distance >= 200.0 {
                enemy.position.x += (speed + (player_position.x - enemy.position.x)) * delta;
                enemy.position.y += (speed + (player_position.y - enemy.position.y)) * delta;
            }
        }
    }

    pub fn spawn_enemies(&mut self, delta: &f32) {
        self.time_since_spawn += delta;

        if self.time_since_spawn >= self.spawn_rate {
            let spawned_enemy = Enemy::new(
                30,
                Position {
                    x: 200.0,
                    y: 200.0,
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
            damage: 10,
            time_since_last_attack: 0.0,
            attack_speed: 1.0,
            position,
            texture,
        }
    }
}
