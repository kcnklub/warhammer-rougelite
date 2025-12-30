use raylib_testing::{game_state, player};

fn main() {
    let (mut rl, thread) = raylib::init()
        .fullscreen()
        .title("Move Block with WASD")
        .build();

    // TODO split game integration loop from FPS so I don't need this.
    rl.set_target_fps(144);

    let texture = rl
        .load_texture(&thread, "./assests/sprites/dark-fighter.png")
        .unwrap();
    let player = player::Player::new(texture);

    let mut game_state = game_state::GameState::new(&mut rl, player);

    while !game_state.rl.window_should_close() {
        let delta = game_state.rl.get_frame_time(); // only get the delta a single time.
        game_state.game_tick(&delta);
        game_state.render(&thread);
    }
}
