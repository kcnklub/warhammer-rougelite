use raylib::prelude::*;

use crate::{enemy::AllEnemies, player::Player, projectile::AllProjectiles};

pub struct GameState<'a> {
    pub rl: &'a mut raylib::RaylibHandle,
    pub player: Player,
    pub projectiles: AllProjectiles<'a>,
    pub enemies: AllEnemies<'a>,
}

impl<'a> GameState<'a> {
    pub fn new(
        rl: &'a mut raylib::RaylibHandle,
        player: Player,
        enemy_texture: &'a Texture2D,
        bullet_texture: &'a Texture2D,
    ) -> Self {
        GameState {
            rl,
            player,
            projectiles: AllProjectiles::new(bullet_texture),
            enemies: AllEnemies::new(enemy_texture),
        }
    }

    pub fn player_alive(&self) -> bool {
        self.player.is_alive()
    }

    pub fn game_tick(&mut self, delta: &f32) {
        self.player.handle_user_input(self.rl, &delta);
        self.player.handle_status_effects(&delta);
        self.player.handle_enemies(&mut self.enemies, &delta);

        // handle and update projectiles
        // TODO I need to clean up projectiles that are passed the end of the play area!!
        let mut new_projectiles = self.player.handle_weapons(&delta);
        self.projectiles.append(&mut new_projectiles);
        self.projectiles.move_projectiles(&delta);
        self.projectiles.handle_collision(&mut self.enemies);

        self.enemies.tick(&self.player.position, &delta);
        self.enemies.spawn_enemies(&delta);
    }
}
