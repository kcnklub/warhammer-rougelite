use raylib::prelude::*;

pub struct Background<'a> {
    texture: &'a Texture2D,
    tile_size: f32,
    parallax_factor: f32,
}

impl<'a> Background<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        Self {
            texture,
            tile_size: 128.0,
            parallax_factor: 0.7,
        }
    }

    pub fn render(&self, d: &mut RaylibMode2D<RaylibDrawHandle>, camera_target: Vector2) {
        // Calculate parallax-adjusted camera position
        let parallax_x = camera_target.x * self.parallax_factor;
        let parallax_y = camera_target.y * self.parallax_factor;

        // Get screen dimensions
        let screen_width = d.get_screen_width() as f32;
        let screen_height = d.get_screen_height() as f32;

        // Calculate visible tile range (with parallax adjustment)
        let start_x = ((parallax_x - screen_width / 2.0) / self.tile_size).floor() as i32 - 1;
        let end_x = ((parallax_x + screen_width / 2.0) / self.tile_size).ceil() as i32 + 1;
        let start_y = ((parallax_y - screen_height / 2.0) / self.tile_size).floor() as i32 - 1;
        let end_y = ((parallax_y + screen_height / 2.0) / self.tile_size).ceil() as i32 + 1;

        // Render visible tiles
        for tile_y in start_y..=end_y {
            for tile_x in start_x..=end_x {
                let world_x = tile_x as f32 * self.tile_size;
                let world_y = tile_y as f32 * self.tile_size;

                d.draw_texture_pro(
                    self.texture,
                    Rectangle::new(0.0, 0.0, self.tile_size, self.tile_size),
                    Rectangle::new(world_x, world_y, self.tile_size, self.tile_size),
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            }
        }
    }
}
