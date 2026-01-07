use warhammer_rougelite::{
    game_state,
    player::{self},
    renderer::render_game_state,
    utils::{Direction, Position},
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
    //rl.set_target_fps(144);

    let player_texture = rl
        .load_texture(&thread, "./assests/sprites/marine.png")
        .unwrap();

    let enemy_texture = rl
        .load_texture(&thread, "./assests/sprites/servo-skull.png")
        .unwrap();

    let bullet_texture = rl
        .load_texture(&thread, "./assests/sprites/bullet_new.png")
        .unwrap();

    let ground_texture = rl
        .load_texture(&thread, "./assests/sprites/ground-tile-01.png")
        .unwrap();

    let ground_texture2 = rl
        .load_texture(&thread, "./assests/sprites/ground-tile-02.png")
        .unwrap();

    let position = Position {
        x: (rl.get_screen_width() / 2) as f32,
        y: (rl.get_screen_height() / 2) as f32,
        direction: Direction::Right,
    };
    let player = player::Player::new(position, player_texture);

    let mut game_state = game_state::GameState::new(
        &mut rl,
        player,
        &enemy_texture,
        &bullet_texture,
        &ground_texture,
        &ground_texture2,
    );

    while !game_state.rl.window_should_close() && game_state.player_alive() {
        let delta = game_state.rl.get_frame_time(); // only get the delta a single time.
        game_state.game_tick(&delta);
        render_game_state(&mut game_state, &thread);
    }
}
