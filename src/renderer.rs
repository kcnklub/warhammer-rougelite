use crate::{
    enemy::AllEnemies, game_state::GameState, player::Player, projectile::AllProjectiles,
    utils::Direction,
};
use raylib::{color::Color, prelude::*};

use crate::projectile::Projectile;

pub fn render_game_state(game_state: &mut GameState, thread: &raylib::RaylibThread) {
    let fps = game_state.rl.get_fps();
    let time = game_state.rl.get_time();

    let mut d = game_state.rl.begin_drawing(thread);

    d.clear_background(Color::DARKGRAY);
    // Draw instructions
    d.draw_text("Use WASD to move", 10, 10, 20, Color::WHITE);
    d.draw_text(&format!("FPS: {}", fps), 400, 10, 20, Color::GREEN);

    render_player(&mut d, &game_state.player);
    render_projectiles(&mut d, &game_state.projectiles, &time);
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
        Direction::Up => 180.0,
        Direction::Down => 0.0,
        Direction::Left => 90.0,
        Direction::Right => 270.0,
    };
    d.draw_circle_lines(
        player.position.x as i32,
        player.position.y as i32,
        (player.texture.width / 2) as f32,
        Color::RED,
    );

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
        let source_rec = ffi::Rectangle {
            x: 0.0,
            y: 0.0,
            width: enemy.texture.width as f32,
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
        let rotation = match enemy.position.direction {
            Direction::Up => 180.0,
            Direction::Down => 0.0,
            Direction::Left => 90.0,
            Direction::Right => 270.0,
        };
        d.draw_texture_pro(
            &enemy.texture,
            source_rec,
            dest_rec,
            origin,
            rotation,
            Color::WHITE,
        );
    }
}

fn render_projectiles(d: &mut RaylibDrawHandle<'_>, projectiles: &AllProjectiles, time: &f64) {
    let active_projectiles = &projectiles.projectiles;
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
                // careful with the clone here, probably doesn't matter but it is strange to have to clone
                let rotation = ((bolter_data.position.x + bolter_data.position.y) * 0.5
                    + time.clone() as f32 * 360.0)
                    % 360.0;
                d.draw_rectangle_pro(rect, origin, rotation, Color::YELLOW);
            }
        }
    }
}
