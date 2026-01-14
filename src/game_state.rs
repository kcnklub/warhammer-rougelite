use raylib::prelude::*;

use crate::{
    enemy::AllEnemies, player::Player, projectile::AllProjectiles, renderer::background::Background,
};

pub const DEBUG_MODE: bool = false;

pub struct GameState<'a> {
    pub rl: &'a mut raylib::RaylibHandle,
    pub player: Player,
    pub projectiles: AllProjectiles<'a>,
    pub enemies: AllEnemies<'a>,
    pub background: Background<'a>,
    pub elapsed_time: f32,
}

impl<'a> GameState<'a> {
    pub fn new(
        rl: &'a mut raylib::RaylibHandle,
        player: Player,
        enemy_texture: &'a Texture2D,
        bullet_texture: &'a Texture2D,
        ground_texture1: &'a Texture2D,
        ground_texture2: &'a Texture2D,
    ) -> Self {
        GameState {
            rl,
            player,
            projectiles: AllProjectiles::new(bullet_texture),
            enemies: AllEnemies::new(enemy_texture),
            background: Background::new(ground_texture1, ground_texture2),
            elapsed_time: 0.0,
        }
    }

    pub fn player_alive(&self) -> bool {
        self.player.is_alive()
    }

    pub fn get_camera(&self, screen_width: i32, screen_height: i32) -> raylib::ffi::Camera2D {
        raylib::ffi::Camera2D {
            target: raylib::ffi::Vector2 {
                x: self.player.position.x,
                y: self.player.position.y,
            },
            offset: raylib::ffi::Vector2 {
                x: (screen_width / 2) as f32,
                y: (screen_height / 2) as f32,
            },
            rotation: 0.0,
            zoom: 1.0,
        }
    }

    pub fn game_tick(&mut self, delta: &f32) {
        self.elapsed_time += delta;

        // Update player aim direction based on mouse BEFORE processing input
        let screen_width = self.rl.get_screen_width();
        let screen_height = self.rl.get_screen_height();
        let camera = self.get_camera(screen_width, screen_height);
        self.player.update_aim_direction(self.rl, camera);

        self.player.handle_user_input(self.rl, &delta);
        self.player.handle_status_effects(&delta);

        // Move enemy tick BEFORE handle_enemies so knockback velocity is applied next frame
        self.enemies.tick(&mut self.player, &delta);
        self.enemies.spawn_enemies(&delta, &self.player.position, self.elapsed_time);

        // handle and update projectiles
        // TODO I need to clean up projectiles that are passed the end of the play area!!
        let mut new_projectiles = self.player.handle_weapons(&delta);
        self.projectiles.append(&mut new_projectiles);
        self.projectiles.move_projectiles(&delta);
        self.projectiles.handle_collision(&mut self.enemies);
    }
}
