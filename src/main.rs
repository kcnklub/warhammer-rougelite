mod main_menu;

use warhammer_rougelite::{
    game_state,
    player::{self},
    renderer::render_game_state,
    utils::Position,
};

use main_menu::{main_menu_tick, MenuAction};

enum AppMode {
    MainMenu,
    InGame,
}

fn main() {
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

    let mut mode = AppMode::MainMenu;
    while !rl.window_should_close() {
        match mode {
            AppMode::MainMenu => {
                rl.show_cursor();
                match main_menu_tick(&mut rl, &thread, &menu_bg_texture) {
                    MenuAction::None => {}
                    MenuAction::Play => mode = AppMode::InGame,
                    MenuAction::Exit => break,
                }
            }
            AppMode::InGame => {
                rl.hide_cursor();
                run_game(&mut rl, &thread);
                mode = AppMode::MainMenu;
            }
        }
    }
}

fn run_game(rl: &mut raylib::RaylibHandle, thread: &raylib::RaylibThread) {
    let player_texture = rl
        .load_texture(thread, "./assests/sprites/marine.png")
        .unwrap();

    let enemy_texture = rl
        .load_texture(thread, "./assests/sprites/servo-skull.png")
        .unwrap();

    let bullet_texture = rl
        .load_texture(thread, "./assests/sprites/bullet_new.png")
        .unwrap();

    let ground_texture = rl
        .load_texture(thread, "./assests/sprites/ground-tile-01.png")
        .unwrap();

    let ground_texture2 = rl
        .load_texture(thread, "./assests/sprites/ground-tile-02.png")
        .unwrap();

    let position = Position {
        x: (rl.get_screen_width() / 2) as f32,
        y: (rl.get_screen_height() / 2) as f32,
    };
    let player = player::Player::new(position, player_texture);

    let mut game_state = game_state::GameState::new(
        rl,
        thread,
        player,
        &enemy_texture,
        &bullet_texture,
        &ground_texture,
        &ground_texture2,
    );

    while !game_state.rl.window_should_close() && game_state.player_alive() {
        let delta = game_state.rl.get_frame_time();
        game_state.game_tick(&delta);
        render_game_state(&mut game_state, thread);
    }
}
