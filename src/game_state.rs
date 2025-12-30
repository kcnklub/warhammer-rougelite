use raylib::{
    color::Color,
    ffi::{self},
    prelude::RaylibDraw,
};

use crate::{
    player::Player,
    projectile::{AllProjectiles, Projectile},
};

pub struct GameState<'a> {
    pub rl: &'a mut raylib::RaylibHandle,
    pub player: Player,
    pub projectiles: AllProjectiles,
}

impl<'a> GameState<'a> {
    pub fn new(rl: &'a mut raylib::RaylibHandle, player: Player) -> Self {
        GameState {
            rl,
            player,
            projectiles: AllProjectiles::new(),
        }
    }

    pub fn game_tick(&mut self, delta: &f32) {
        self.player.handle_movement(self.rl, &delta);
        self.player.handle_status_effects(&delta);

        // handle and update projectiles
        // TODO I need to clean up projectiles that are passed the end of the play area!!
        let mut new_projectiles = self.player.handle_weapons(&delta);
        self.projectiles.append(&mut new_projectiles);
        self.projectiles.move_projectiles(&delta);
    }

    pub fn render(&mut self, thread: &raylib::RaylibThread) {
        let fps = self.rl.get_fps();
        let time = self.rl.get_time();

        let mut d = self.rl.begin_drawing(thread);

        d.clear_background(Color::DARKGRAY);

        let player = &self.player;

        let source_rec = ffi::Rectangle {
            x: 0.0,
            y: 0.0,
            width: player.texture.width as f32,
            height: player.texture.height as f32,
        };
        let dest_rec = ffi::Rectangle {
            x: player.position.x,
            y: player.position.y,
            width: player.texture.width as f32,
            height: player.texture.height as f32,
        };
        let origin = ffi::Vector2 {
            x: player.texture.width as f32 / 2.0,
            y: player.texture.height as f32 / 2.0,
        };
        let rotation = match player.position.direction {
            crate::player::Direction::Up => 180.0,
            crate::player::Direction::Down => 0.0,
            crate::player::Direction::Left => 90.0,
            crate::player::Direction::Right => 270.0,
        };
        d.draw_texture_pro(
            &player.texture,
            source_rec,
            dest_rec,
            origin,
            rotation,
            Color::WHITE,
        );

        // Draw instructions
        d.draw_text("Use WASD to move", 10, 10, 20, Color::WHITE);
        d.draw_text(&format!("FPS: {}", fps), 400, 10, 20, Color::GREEN);

        // Draw health with color coding
        let health_color = if player.health < 30.0 {
            Color::RED
        } else if player.health < 60.0 {
            Color::YELLOW
        } else {
            Color::WHITE
        };
        d.draw_text(
            &format!("Health: {:.0}/{:.0}", player.health, player.max_health),
            10,
            30,
            20,
            health_color,
        );

        // Draw active status effects
        let active_statuses = player.get_active_status_names();
        d.draw_text(
            &format!("Active Status Effects: {}", active_statuses.len()),
            10,
            50,
            20,
            Color::LIGHTGRAY,
        );

        let mut y_offset = 70;
        for (name, duration) in active_statuses {
            let status_color = match name.as_str() {
                "Poison" | "Burn" | "Slow" | "Stun" => Color::RED,
                "Regeneration" | "Speed Boost" => Color::GREEN,
                _ => Color::WHITE,
            };

            d.draw_text(
                &format!("  {} ({:.1}s)", name, duration),
                10,
                y_offset,
                18,
                status_color,
            );
            y_offset += 20;
        }

        let active_projectiles = &self.projectiles.projectiles;
        for projetile in active_projectiles {
            match projetile {
                Projectile::Bolter(bolter_data) => {
                    let rect = ffi::Rectangle {
                        x: bolter_data.position.x,
                        y: bolter_data.position.y,
                        width: 14.0,
                        height: 14.0,
                    };
                    let origin = ffi::Vector2 { x: 7.0, y: 7.0 };
                    // Rotation based on distance traveled (position sum) and time
                    let rotation = ((bolter_data.position.x + bolter_data.position.y) * 0.5
                        + time as f32 * 360.0)
                        % 360.0;
                    d.draw_rectangle_pro(rect, origin, rotation, Color::BLACK);
                }
            }
        }
    }
}
