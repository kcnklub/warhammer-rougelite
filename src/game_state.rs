use raylib::prelude::{RaylibShader, *};

use crate::{
    enemy::AllEnemies, player::Player, projectiles::AllProjectiles,
    renderer::background::Background,
};

pub const DEBUG_MODE: bool = true;

pub struct GameState<'a> {
    pub rl: &'a mut raylib::RaylibHandle,
    pub player: Player,
    pub projectiles: AllProjectiles<'a>,
    pub enemies: AllEnemies<'a>,
    pub background: Background<'a>,
    pub white_texture: Texture2D,
    pub multi_melta_shader: MultiMeltaShader,
    pub elapsed_time: f32,
}

pub struct MultiMeltaShader {
    pub shader: Shader,
    pub time_loc: i32,
    pub noise_scale_loc: i32,
    pub intensity_loc: i32,
    pub alpha_loc: i32,
    pub color_hot_loc: i32,
    pub color_mid_loc: i32,
    pub color_cool_loc: i32,
}

impl<'a> GameState<'a> {
    pub fn new(
        rl: &'a mut raylib::RaylibHandle,
        thread: &raylib::RaylibThread,
        player: Player,
        enemy_texture: &'a Texture2D,
        bullet_texture: &'a Texture2D,
        ground_texture1: &'a Texture2D,
        ground_texture2: &'a Texture2D,
    ) -> Self {
        let multi_melta_shader =
            rl.load_shader(thread, None, Some("./assests/shaders/multi_melta_flame.fs"));
        if !multi_melta_shader.is_shader_valid() {
            panic!("Multi Melta shader failed to load");
        }

        let white_image = Image::gen_image_color(1, 1, Color::WHITE);
        let white_texture = rl
            .load_texture_from_image(thread, &white_image)
            .expect("failed to create white texture");
        let time_loc = multi_melta_shader.get_shader_location("time");
        let noise_scale_loc = multi_melta_shader.get_shader_location("noise_scale");
        let intensity_loc = multi_melta_shader.get_shader_location("intensity");
        let alpha_loc = multi_melta_shader.get_shader_location("alpha");
        let color_hot_loc = multi_melta_shader.get_shader_location("color_hot");
        let color_mid_loc = multi_melta_shader.get_shader_location("color_mid");
        let color_cool_loc = multi_melta_shader.get_shader_location("color_cool");

        GameState {
            rl,
            player,
            projectiles: AllProjectiles::new(bullet_texture),
            enemies: AllEnemies::new(enemy_texture),
            background: Background::new(ground_texture1, ground_texture2),
            white_texture,
            multi_melta_shader: MultiMeltaShader {
                shader: multi_melta_shader,
                time_loc,
                noise_scale_loc,
                intensity_loc,
                alpha_loc,
                color_hot_loc,
                color_mid_loc,
                color_cool_loc,
            },
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
        self.enemies
            .spawn_enemies(&delta, &self.player.position, self.elapsed_time);

        // handle and update projectiles
        // TODO I need to clean up projectiles that are passed the end of the play area!!
        let mut new_projectiles = self.player.handle_weapons(&delta);
        self.projectiles.append(&mut new_projectiles);
        self.projectiles.move_projectiles(&self.player, &delta);
        self.projectiles.handle_collision(&mut self.enemies);
    }
}
