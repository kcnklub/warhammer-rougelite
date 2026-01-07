use crate::{
    enemy::AllEnemies,
    game_state::{self, GameState},
    player::Player,
    projectile::AllProjectiles,
    utils::Direction,
};
use raylib::{color::Color, prelude::*};

use crate::projectile::Projectile;

pub mod background;

// higher the smaller
// TODO fix the reverse scaling as it is just odd
const PLAYER_SCALE: f32 = 1.5;

pub fn render_game_state(game_state: &mut GameState, thread: &raylib::RaylibThread) {
    let fps = game_state.rl.get_fps();
    let _time = game_state.rl.get_time();

    let mut d = game_state.rl.begin_drawing(thread);

    d.clear_background(Color::BLACK);

    let camera = ffi::Camera2D {
        target: ffi::Vector2 {
            x: game_state.player.position.x,
            y: game_state.player.position.y,
        },
        offset: ffi::Vector2 {
            x: (d.get_screen_width() / 2) as f32,
            y: (d.get_screen_height() / 2) as f32,
        },
        rotation: 0.0,
        zoom: 1.0,
    };

    // === WORLD SPACE RENDERING (with camera) ===
    {
        let mut d2 = d.begin_mode2D(camera);

        // Background (parallax layer)
        let camera_target =
            Vector2::new(game_state.player.position.x, game_state.player.position.y);
        game_state.background.render(&mut d2, camera_target);

        // Game entities (normal layer)
        render_player(&mut d2, &game_state.player);
        render_projectiles(&mut d2, &game_state.projectiles);
        render_enemies(&mut d2, &game_state.enemies);
    }

    // === SCREEN SPACE UI (after camera - draws on top) ===
    d.draw_text("Use WASD to move", 10, 10, 20, Color::WHITE);
    d.draw_text(&format!("FPS: {}", fps), 400, 10, 20, Color::GREEN);

    // Game Clock - centered at top
    let minutes = (game_state.elapsed_time / 60.0) as i32;
    let seconds = (game_state.elapsed_time % 60.0) as i32;
    let time_text = format!("{:02}:{:02}", minutes, seconds);
    let screen_width = d.get_screen_width();
    let text_width = 60; // Approximate width of "MM:SS" at font size 20
    let clock_x = (screen_width / 2) - (text_width / 2);
    d.draw_text(&time_text, clock_x, 10, 20, Color::WHITE);

    render_player_ui(&mut d, &game_state.player);
}

fn render_player(d: &mut RaylibMode2D<RaylibDrawHandle>, player: &Player) {
    let source_width = match player.position.direction {
        Direction::Up => player.texture.width as f32,
        Direction::Down => player.texture.width as f32,
        Direction::Left => -1.0 * player.texture.width as f32,
        Direction::Right => player.texture.width as f32,
    };
    let source_rec = Rectangle::new(0.0, 0.0, source_width, player.texture.height as f32);
    let dest_rec = Rectangle::new(
        player.position.x,
        player.position.y,
        player.texture.width as f32 / PLAYER_SCALE,
        player.texture.height as f32 / PLAYER_SCALE,
    );
    let origin = Vector2::new(
        player.texture.width as f32 / (PLAYER_SCALE * 2.0),
        player.texture.height as f32 / (PLAYER_SCALE * 2.0),
    );
    let rotation = match player.position.direction {
        Direction::Up => 0.0,
        Direction::Down => 0.0,
        Direction::Left => 0.0,
        Direction::Right => 0.0,
    };
    if game_state::DEBUG_MODE {
        d.draw_circle_lines(
            player.position.x as i32,
            player.position.y as i32,
            (player.texture.width / (PLAYER_SCALE * 2.0) as i32) as f32,
            Color::RED,
        );
    }

    d.draw_texture_pro(
        &player.texture,
        source_rec,
        dest_rec,
        origin,
        rotation,
        Color::WHITE,
    );
}

fn render_player_ui(d: &mut RaylibDrawHandle, player: &Player) {
    // Draw health with color coding
    let health_color = if player.health < 30 {
        Color::RED
    } else if player.health < 60 {
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
}

pub fn render_enemies(d: &mut RaylibMode2D<RaylibDrawHandle>, enemies: &AllEnemies) {
    for enemy in &enemies.enemies {
        let texture = enemies
            .texture_map
            .get(&enemy.enemy_type)
            .expect("No texture found for enemy");
        let source_width = match enemy.position.direction {
            Direction::Up => texture.width as f32,
            Direction::Down => texture.width as f32,
            Direction::Left => texture.width as f32,
            Direction::Right => -1.0 * texture.width as f32,
        };

        let source_rec = Rectangle::new(0.0, 0.0, source_width, texture.height as f32);
        let dest_rec = Rectangle::new(
            enemy.position.x,
            enemy.position.y,
            texture.width as f32,
            texture.height as f32,
        );
        let origin = Vector2::new(texture.width as f32 / 2.0, texture.height as f32 / 2.0);
        d.draw_texture_pro(&texture, source_rec, dest_rec, origin, 0.0, Color::WHITE);

        if game_state::DEBUG_MODE {
            let debug_rect = Rectangle::new(
                enemy.position.x - origin.x,
                enemy.position.y - origin.y,
                texture.width as f32,
                texture.height as f32,
            );
            d.draw_rectangle_lines_ex(debug_rect, 2.0, Color::RED);
        }
    }
}

fn render_projectiles(d: &mut RaylibMode2D<RaylibDrawHandle>, projectiles: &AllProjectiles) {
    let active_projectiles = &projectiles.projectiles;
    for projetile in active_projectiles {
        match projetile {
            Projectile::Bolter(bolter_data) => {
                let source_rec = Rectangle::new(
                    0.0,
                    0.0,
                    projectiles.texture.width as f32,
                    projectiles.texture.height as f32,
                );
                let dest_rec = Rectangle::new(
                    bolter_data.position.x,
                    bolter_data.position.y,
                    projectiles.texture.width as f32 / 2.0,
                    projectiles.texture.height as f32 / 2.0,
                );
                let origin = Vector2::new(
                    projectiles.texture.width as f32 / 4.0,
                    projectiles.texture.height as f32 / 4.0,
                );

                let rotation = match bolter_data.position.direction {
                    Direction::Up => 270.0,
                    Direction::Down => 90.0,
                    Direction::Left => 180.0,
                    Direction::Right => 0.0,
                };
                d.draw_texture_pro(
                    &projectiles.texture,
                    source_rec,
                    dest_rec,
                    origin,
                    rotation,
                    Color::WHITE,
                );
                if game_state::DEBUG_MODE {
                    if game_state::DEBUG_MODE {
                        let debug_rect = Rectangle::new(
                            bolter_data.position.x - origin.x,
                            bolter_data.position.y - origin.y,
                            projectiles.texture.width as f32 / 2.0,
                            projectiles.texture.height as f32 / 2.0,
                        );
                        d.draw_rectangle_lines_ex(debug_rect, 2.0, Color::RED);
                    }
                }
            }
        }
    }
}
