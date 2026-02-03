use crate::{
    game_state,
    main_menu::{main_menu_tick, MenuAction},
    player,
    renderer::render_game_state,
    utils::Position,
};

enum AppMode {
    MainMenu,
    InGame,
}

pub struct WarhammerGame {
    rl: raylib::RaylibHandle,
    thread: raylib::RaylibThread,
    menu_bg_texture: raylib::texture::Texture2D,
    mode: AppMode,
}

impl WarhammerGame {
    pub fn new() -> Self {
        // TODO make it so that I can detect/set this resolution via a config.
        let (mut rl, thread) = raylib::init()
            .fullscreen()
            .width(2480)
            .height(1440)
            .title("Move Block with WASD")
            .build();

        // TODO split game integration loop from FPS so I don't need this.
        // rl.set_target_fps(144);

        let menu_bg_texture = rl
            .load_texture(&thread, "./assests/sprites/ground-tile-01.png")
            .unwrap();

        Self {
            rl,
            thread,
            menu_bg_texture,
            mode: AppMode::MainMenu,
        }
    }

    pub fn start_game(&mut self) {
        while !self.rl.window_should_close() {
            match self.mode {
                AppMode::MainMenu => {
                    self.rl.show_cursor();
                    match main_menu_tick(&mut self.rl, &self.thread, &self.menu_bg_texture) {
                        MenuAction::None => {}
                        MenuAction::Play => self.mode = AppMode::InGame,
                        MenuAction::Exit => break,
                    }
                }
                AppMode::InGame => {
                    self.rl.hide_cursor();
                    self.run_game();
                    self.mode = AppMode::MainMenu;
                }
            }
        }
    }

    fn run_game(&mut self) {
        let player_texture = self
            .rl
            .load_texture(&self.thread, "./assests/sprites/marine.png")
            .unwrap();

        let enemy_texture = self
            .rl
            .load_texture(&self.thread, "./assests/sprites/servo-skull.png")
            .unwrap();

        let bullet_texture = self
            .rl
            .load_texture(&self.thread, "./assests/sprites/bullet_new.png")
            .unwrap();

        let ground_texture = self
            .rl
            .load_texture(&self.thread, "./assests/sprites/ground-tile-01.png")
            .unwrap();

        let ground_texture2 = self
            .rl
            .load_texture(&self.thread, "./assests/sprites/ground-tile-02.png")
            .unwrap();

        let position = Position {
            x: (self.rl.get_screen_width() / 2) as f32,
            y: (self.rl.get_screen_height() / 2) as f32,
        };
        let player = player::Player::new(position, player_texture);

        let mut game_state = game_state::GameState::new(
            &mut self.rl,
            &self.thread,
            player,
            &enemy_texture,
            &bullet_texture,
            &ground_texture,
            &ground_texture2,
        );

        while !game_state.rl.window_should_close() && game_state.player_alive() {
            let delta = game_state.rl.get_frame_time();
            game_state.game_tick(&delta);
            render_game_state(&mut game_state, &self.thread);
        }
    }
}
