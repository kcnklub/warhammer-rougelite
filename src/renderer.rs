use crate::{
    enemy::AllEnemies,
    game_state::{self, GameState},
    player::Player,
    projectile::AllProjectiles,
    utils::Direction,
};
use raylib::{color::Color, prelude::*};

use crate::projectile::Projectile;

pub fn render_game_state(game_state: &mut GameState, thread: &raylib::RaylibThread) {
    let fps = game_state.rl.get_fps();
    let _time = game_state.rl.get_time();

    let mut d = game_state.rl.begin_drawing(thread);

    d.clear_background(Color::DARKBROWN);
    // Draw instructions
    d.draw_text("Use WASD to move", 10, 10, 20, Color::WHITE);
    d.draw_text(&format!("FPS: {}", fps), 400, 10, 20, Color::GREEN);

    render_player(&mut d, &game_state.player);
    render_projectiles(&mut d, &game_state.projectiles);
    render_enemies(&mut d, &game_state.enemies);
}

fn render_player(d: &mut RaylibDrawHandle<'_>, player: &Player) {
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
        Direction::Up => 0.0,
        Direction::Down => 0.0,
        Direction::Left => 0.0,
        Direction::Right => 0.0,
    };
    if game_state::DEBUG_MODE {
        d.draw_circle_lines(
            player.position.x as i32,
            player.position.y as i32,
            (player.texture.width / 2) as f32,
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

pub fn render_enemies(d: &mut RaylibDrawHandle<'_>, enemies: &AllEnemies) {
    for enemy in &enemies.enemies {
        let source_width = match enemy.position.direction {
            Direction::Up => enemy.texture.width as f32,
            Direction::Down => enemy.texture.width as f32,
            Direction::Left => enemy.texture.width as f32,
            Direction::Right => -1.0 * enemy.texture.width as f32,
        };

        let source_rec = ffi::Rectangle {
            x: 0.0,
            y: 0.0,
            width: source_width,
            height: enemy.texture.height as f32,
        };
        let dest_rec = ffi::Rectangle {
            x: enemy.position.x,
            y: enemy.position.y,
            width: enemy.texture.width as f32,
            height: enemy.texture.height as f32,
        };
        let origin = ffi::Vector2 {
            x: enemy.texture.width as f32 / 2.0,
            y: enemy.texture.height as f32 / 2.0,
        };
        d.draw_texture_pro(
            &enemy.texture,
            source_rec,
            dest_rec,
            origin,
            0.0,
            Color::WHITE,
        );

        if game_state::DEBUG_MODE {
            let starting_width = enemy.texture.width as f32;
            d.draw_rectangle(
                enemy.position.x as i32,
                enemy.position.y as i32,
                starting_width as i32,
                5,
                Color::DARKGRAY,
            );

            let ratio = enemy.health as f32 / enemy.max_health as f32;
            let current_width = starting_width * ratio;
            d.draw_rectangle(
                enemy.position.x as i32,
                enemy.position.y as i32,
                current_width as i32,
                5,
                Color::RED,
            );
        }
    }
}

fn render_projectiles(d: &mut RaylibDrawHandle<'_>, projectiles: &AllProjectiles) {
    let active_projectiles = &projectiles.projectiles;
    for projetile in active_projectiles {
        match projetile {
            Projectile::Bolter(bolter_data) => {
                let source_rec = ffi::Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: projectiles.texture.width as f32,
                    height: projectiles.texture.height as f32,
                };
                let dest_rec = ffi::Rectangle {
                    x: bolter_data.position.x,
                    y: bolter_data.position.y,
                    width: projectiles.texture.width as f32,
                    height: projectiles.texture.height as f32,
                };
                let origin = ffi::Vector2 {
                    x: projectiles.texture.width as f32 / 2.0,
                    y: projectiles.texture.height as f32 / 2.0,
                };

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
                    d.draw_rectangle_lines_ex(dest_rec, 5.0, Color::RED);
                }
            }
        }
    }
}
