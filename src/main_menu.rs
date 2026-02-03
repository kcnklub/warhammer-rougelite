use raylib::{
    color::Color,
    consts::MouseButton,
    math::Rectangle,
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibHandle, RaylibThread, Texture2D, Vector2},
};

pub enum MenuAction {
    None,
    Play,
    Exit,
}

pub fn main_menu_tick(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    background_tile: &Texture2D,
) -> MenuAction {
    let screen_w = rl.get_screen_width();
    let screen_h = rl.get_screen_height();
    let mouse = rl.get_mouse_position();
    let click = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

    let title = "WARHAMMER ROUGELITE";
    let title_size = (screen_h as f32 * 0.07).clamp(42.0, 84.0) as i32;
    let title_w = rl.measure_text(title, title_size);
    let title_x = (screen_w - title_w) / 2;
    let title_y = (screen_h as f32 * 0.18) as i32;

    let subtitle = "Click Play to begin";
    let subtitle_size = 20;
    let subtitle_w = rl.measure_text(subtitle, subtitle_size);
    let subtitle_x = (screen_w - subtitle_w) / 2;
    let subtitle_y = title_y + title_size + 18;

    let option_size = (screen_h as f32 * 0.055).clamp(28.0, 56.0) as i32;
    let gap = (option_size as f32 * 1.25).clamp(44.0, 78.0);
    let options_top = (screen_h as f32 * 0.44) as i32;

    let play_text = "PLAY";
    let exit_text = "EXIT";
    let play_w = rl.measure_text(play_text, option_size);
    let exit_w = rl.measure_text(exit_text, option_size);
    let play_x = (screen_w - play_w) / 2;
    let exit_x = (screen_w - exit_w) / 2;
    let play_y = options_top;
    let exit_y = options_top + gap as i32;

    // Slightly generous hitboxes so clicking feels good.
    let pad_x = (option_size as f32 * 0.45).clamp(14.0, 26.0);
    let pad_y = (option_size as f32 * 0.35).clamp(10.0, 18.0);
    let play_rect = Rectangle::new(
        (play_x as f32) - pad_x,
        (play_y as f32) - pad_y,
        (play_w as f32) + pad_x * 2.0,
        (option_size as f32) + pad_y * 2.0,
    );
    let exit_rect = Rectangle::new(
        (exit_x as f32) - pad_x,
        (exit_y as f32) - pad_y,
        (exit_w as f32) + pad_x * 2.0,
        (option_size as f32) + pad_y * 2.0,
    );

    let hover_play = play_rect.check_collision_point_rec(mouse);
    let hover_exit = exit_rect.check_collision_point_rec(mouse);

    let mut d = rl.begin_drawing(thread);

    d.clear_background(Color::BLACK);
    draw_tiled_background(&mut d, background_tile, screen_w, screen_h);

    // Darken a bit for readability and add a rough vignette.
    d.draw_rectangle(0, 0, screen_w, screen_h, Color::new(0, 0, 0, 120));
    d.draw_rectangle_gradient_v(
        0,
        0,
        screen_w,
        screen_h,
        Color::new(0, 0, 0, 80),
        Color::new(0, 0, 0, 120),
    );

    d.draw_text(title, title_x, title_y, title_size, Color::RAYWHITE);
    d.draw_text(
        subtitle,
        subtitle_x,
        subtitle_y,
        subtitle_size,
        Color::new(180, 190, 200, 255),
    );

    let option_base = Color::new(230, 224, 210, 255);
    let option_hover = Color::new(210, 40, 35, 255);
    d.draw_text(
        play_text,
        play_x,
        play_y,
        option_size,
        if hover_play {
            option_hover
        } else {
            option_base
        },
    );
    d.draw_text(
        exit_text,
        exit_x,
        exit_y,
        option_size,
        if hover_exit {
            option_hover
        } else {
            option_base
        },
    );

    // Subtle underline on hover (still text-forward, but punchier).
    if hover_play {
        let underline_y = play_y + option_size + 6;
        d.draw_rectangle(play_x, underline_y, play_w, 3, option_hover);
    }
    if hover_exit {
        let underline_y = exit_y + option_size + 6;
        d.draw_rectangle(exit_x, underline_y, exit_w, 3, option_hover);
    }

    if click {
        if hover_play {
            return MenuAction::Play;
        }
        if hover_exit {
            return MenuAction::Exit;
        }
    }

    MenuAction::None
}

fn draw_tiled_background(d: &mut RaylibDrawHandle, tile: &Texture2D, screen_w: i32, screen_h: i32) {
    let tile_w = tile.width.max(1);
    let tile_h = tile.height.max(1);

    // Tint down to feel grittier.
    let tint = Color::new(170, 170, 170, 255);

    let mut y = 0;
    while y < screen_h {
        let mut x = 0;
        while x < screen_w {
            d.draw_texture(tile, x, y, tint);
            x += tile_w;
        }
        y += tile_h;
    }

    // A few subtle "scratches" to break up the clean tiling.
    let scratch = Color::new(0, 0, 0, 35);
    d.draw_line_ex(
        Vector2::new(screen_w as f32 * 0.12, screen_h as f32 * 0.18),
        Vector2::new(screen_w as f32 * 0.74, screen_h as f32 * 0.10),
        3.0,
        scratch,
    );
    d.draw_line_ex(
        Vector2::new(screen_w as f32 * 0.08, screen_h as f32 * 0.76),
        Vector2::new(screen_w as f32 * 0.92, screen_h as f32 * 0.64),
        2.0,
        scratch,
    );
    d.draw_line_ex(
        Vector2::new(screen_w as f32 * 0.20, screen_h as f32 * 0.56),
        Vector2::new(screen_w as f32 * 0.68, screen_h as f32 * 0.60),
        1.5,
        scratch,
    );
}
