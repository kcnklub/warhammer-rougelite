use raylib::prelude::*;

pub struct Background<'a> {
    texture1: &'a Texture2D,
    texture2: &'a Texture2D,
    texture_size: f32,
    tile_size: f32,
    parallax_factor: f32,
}

impl<'a> Background<'a> {
    pub fn new(texture1: &'a Texture2D, texture2: &'a Texture2D) -> Self {
        Self {
            texture1,
            texture2,
            texture_size: 128.0,
            tile_size: 512.0,
            parallax_factor: 0.7,
        }
    }

    // Deterministic hash function to choose tile based on coordinates
    fn select_tile(&self, tile_x: i32, tile_y: i32) -> &Texture2D {
        // Simple hash using prime number multiplication for good distribution
        let hash =
            ((tile_x.wrapping_mul(374761393)).wrapping_add(tile_y.wrapping_mul(668265263))) as u32;

        // Use hash to pick texture (50/50 split)
        if hash % 2 == 0 {
            self.texture1
        } else {
            self.texture2
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

                // Select texture based on tile coordinates
                let texture = self.select_tile(tile_x, tile_y);

                d.draw_texture_pro(
                    texture,
                    Rectangle::new(0.0, 0.0, self.texture_size, self.texture_size),
                    Rectangle::new(world_x, world_y, self.tile_size, self.tile_size),
                    Vector2::zero(),
                    0.0,
                    Color::WHITE,
                );
            }
        }
    }
}
