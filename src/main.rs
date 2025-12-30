use raylib_testing::{
    game_state,
    player::{self, Direction, Position},
};

fn main() {
    // TODO make it so that I can detect/set this resolution via a config.
    let (mut rl, thread) = raylib::init()
        .fullscreen()
        .width(2480)
        .height(1440)
        .title("Move Block with WASD")
        .build();

    // TODO split game integration loop from FPS so I don't need this.
    rl.set_target_fps(144);

    let texture = rl
        .load_texture(&thread, "./assests/sprites/dark-fighter.png")
        .unwrap();

    let position = Position {
        x: (rl.get_screen_width() / 2) as f32,
        y: (rl.get_screen_height() / 2) as f32,
        direction: Direction::Right,
    };
    let player = player::Player::new(position, texture);

    let mut game_state = game_state::GameState::new(&mut rl, player);

    while !game_state.rl.window_should_close() {
        let delta = game_state.rl.get_frame_time(); // only get the delta a single time.
        game_state.game_tick(&delta);
        game_state.render(&thread);
    }
}
