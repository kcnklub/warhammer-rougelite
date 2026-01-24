use crate::{
    enemy::AllEnemies,
    player::Player,
    utils::{Direction, Position},
};

use raylib::prelude::*;

#[derive(Clone, Copy)]
pub struct PowerSwordProjectile {
    pub damage: i32,
    pub position: Position,
    pub direction: Direction,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub width: f32,
    pub height: f32,
    pub slash_distance: f32,
}

impl PowerSwordProjectile {
    pub fn new(position: Position, direction: Direction) -> Self {
        PowerSwordProjectile {
            damage: 25,
            position,
            direction,
            lifetime: 0.25,
            max_lifetime: 0.25,
            width: 120.0,
            height: 20.0,
            slash_distance: 250.0,
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
        match self.direction {
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

    pub fn handle_move(&mut self, player: &Player, delta: &f32) {
        let rotation = match player.moving_direction {
            Direction::Up => 1.0,
            Direction::Down => 1.0,
            Direction::Left => -1.0,
            Direction::Right => 1.0,
        };
        let x_offset = (player.texture.width / 2) as f32;
        self.position = Position {
            x: player.position.x + (x_offset * rotation),
            y: player.position.y,
        };
        self.direction = player.moving_direction;
        self.lifetime -= delta;
    }

    pub fn handle_collision(&mut self, all_enemies: &mut AllEnemies) {
        for enemy in all_enemies.enemies.iter_mut() {
            let texture = all_enemies
                .texture_map
                .get(&enemy.enemy_type)
                .expect("unable to find texture");
            let half_width = texture.width as f32 / 2.0;
            let half_height = texture.height as f32 / 2.0;
            let enemy_rec = Rectangle {
                x: enemy.position.x - half_width,
                y: enemy.position.y - half_height,
                width: texture.width as f32,
                height: texture.height as f32,
            };
            let sword_rect = self.get_collision_rect();
            if enemy_rec.check_collision_recs(&sword_rect) {
                enemy.health -= self.damage;
                println!("Enemy Health: {}", enemy.health);
            }
        }
    }
}
