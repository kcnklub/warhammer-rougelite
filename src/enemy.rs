use std::collections::HashMap;

use rand::prelude::*;
use raylib::prelude::*;

use crate::{
    player::Player,
    utils::{Direction, Position},
};

const SPEED: f32 = 2000.0;

// Spawn rate scaling constants
const BASE_SPAWN_RATE: f32 = 3.0; // Starting spawn interval (seconds)
const MIN_SPAWN_RATE: f32 = 0.3; // Minimum spawn interval (floor)
const SPAWN_SCALING_FACTOR: f32 = 60.0; // How quickly difficulty ramps up

// Viewport and spawn positioning
const SCREEN_HALF_WIDTH: f32 = 1240.0; // 2480 / 2
const SCREEN_HALF_HEIGHT: f32 = 720.0; // 1440 / 2
const SPAWN_BUFFER: f32 = 100.0; // Pixels outside viewport to spawn

#[derive(Clone, Copy)]
enum SpawnEdge {
    Top,
    Bottom,
    Left,
    Right,
}

impl SpawnEdge {
    fn random(rng: &mut impl Rng) -> Self {
        match rng.random_range(0..4) {
            0 => SpawnEdge::Top,
            1 => SpawnEdge::Bottom,
            2 => SpawnEdge::Left,
            _ => SpawnEdge::Right,
        }
    }
}

fn calculate_spawn_position(player_pos: &Position, rng: &mut impl Rng) -> Position {
    let edge = SpawnEdge::random(rng);

    // Calculate viewport bounds in world space
    let view_left = player_pos.x - SCREEN_HALF_WIDTH;
    let view_right = player_pos.x + SCREEN_HALF_WIDTH;
    let view_top = player_pos.y - SCREEN_HALF_HEIGHT;
    let view_bottom = player_pos.y + SCREEN_HALF_HEIGHT;

    let (x, y) = match edge {
        SpawnEdge::Top => {
            let x = rng.random_range(view_left..view_right);
            let y = view_top - SPAWN_BUFFER;
            (x, y)
        }
        SpawnEdge::Bottom => {
            let x = rng.random_range(view_left..view_right);
            let y = view_bottom + SPAWN_BUFFER;
            (x, y)
        }
        SpawnEdge::Left => {
            let x = view_left - SPAWN_BUFFER;
            let y = rng.random_range(view_top..view_bottom);
            (x, y)
        }
        SpawnEdge::Right => {
            let x = view_right + SPAWN_BUFFER;
            let y = rng.random_range(view_top..view_bottom);
            (x, y)
        }
    };

    Position { x, y }
}

pub struct AllEnemies<'a> {
    pub enemies: Vec<Enemy>,
    time_since_spawn: f32,
    pub texture_map: HashMap<EnemyType, &'a Texture2D>,
}

impl<'a> AllEnemies<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        let mut texture_map = HashMap::new();
        texture_map.insert(EnemyType::servo_skull_type(), texture);

        Self {
            enemies: vec![],
            time_since_spawn: 0.0,
            texture_map,
        }
    }

    pub fn tick(&mut self, player: &mut Player, delta: &f32) {
        // retain all alive enemies
        self.enemies.retain(|enemy| enemy.health > 0);

        for mut enemy in self.enemies.iter_mut() {
            handle_movement(player, &mut enemy, delta);
            handle_player_collision(&self.texture_map, player, &mut enemy, delta);
        }
    }

    fn calculate_spawn_interval(&self, elapsed_time: f32) -> f32 {
        let dynamic_rate = BASE_SPAWN_RATE / (1.0 + elapsed_time / SPAWN_SCALING_FACTOR);
        dynamic_rate.max(MIN_SPAWN_RATE)
    }

    pub fn spawn_enemies(&mut self, delta: &f32, player_pos: &Position, elapsed_time: f32) {
        self.time_since_spawn += delta;

        let current_spawn_interval = self.calculate_spawn_interval(elapsed_time);

        if self.time_since_spawn >= current_spawn_interval {
            let mut rng = rand::rng();
            let spawn_position = calculate_spawn_position(player_pos, &mut rng);

            let spawned_enemy = EnemyType::new_servo_skull(spawn_position);
            self.enemies.push(spawned_enemy);
            self.time_since_spawn = 0.0;
        }
    }
}

fn handle_movement(player: &Player, enemy: &mut Enemy, delta: &f32) {
    // Semi-implicit Euler integration

    // Update knockback cooldown timer
    if enemy.knockback_cooldown > 0.0 {
        enemy.knockback_cooldown -= delta;
    }

    // Step 1: Apply friction to velocity
    let friction = 1.0; // Aggressive friction per frame
    enemy.velocity_x *= friction;
    enemy.velocity_y *= friction;

    // Step 2: Calculate direction vector to player
    let dx = player.position.x - enemy.position.x;
    let dy = player.position.y - enemy.position.y;

    // Calculate distance (magnitude of direction vector)
    let distance = (dx * dx + dy * dy).sqrt();

    // Step 3: Calculate acceleration and update velocity
    // (only if distance > 0 to avoid division by zero AND not in knockback state)
    if distance > 0.0 && enemy.knockback_cooldown <= 0.0 {
        let acceleration_x = (dx / distance) * SPEED;
        let acceleration_y = (dy / distance) * SPEED;

        // Step 4: Update velocity with acceleration (semi-implicit Euler!)
        enemy.velocity_x += acceleration_x * delta;
        enemy.velocity_y += acceleration_y * delta;

        enemy.velocity_x = 250.0_f32.min(enemy.velocity_x).max(-250.0);
        enemy.velocity_y = 250.0_f32.min(enemy.velocity_y).max(-250.0);
    }

    // Step 5: Update position using NEW velocity
    enemy.position.x += enemy.velocity_x * delta;
    enemy.position.y += enemy.velocity_y * delta;

    // Step 6: Update direction based on velocity
    if enemy.velocity_x.abs() > 0.1 {
        enemy.direction = if enemy.velocity_x > 0.0 {
            Direction::Right
        } else {
            Direction::Left
        };
    }
}

fn handle_player_collision<'a>(
    texture_map: &HashMap<EnemyType, &'a Texture2D>,
    player: &mut Player,
    enemy: &mut Enemy,
    delta: &f32,
) {
    let texture = texture_map
        .get(&enemy.enemy_type)
        .expect("unable to find texture");

    // Center the collision rectangle on enemy.position to match visual rendering
    // (renderer uses origin offset to center sprite on position)
    let half_width = texture.width as f32 / 2.0;
    let half_height = texture.height as f32 / 2.0;
    let enemy_rec = Rectangle::new(
        enemy.position.x - half_width,
        enemy.position.y - half_height,
        texture.width as f32,
        texture.height as f32,
    );
    let player_point = Vector2::new(player.position.x, player.position.y);

    enemy.time_since_last_attack += delta;
    if enemy_rec.check_collision_circle_rec(player_point, player.collision_radius) {
        if enemy.time_since_last_attack >= enemy.attack_speed {
            player.health -= enemy.damage;
            enemy.time_since_last_attack = 0.0;
        }
        // enemy.position IS the center (matches rendering origin)
        let dx = enemy.position.x - player.position.x;
        let dy = enemy.position.y - player.position.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 0.0 {
            let knockback_strength = 20000.0;
            let knockback_duration = 0.2;

            enemy.velocity_x += (dx / distance) * knockback_strength * delta;
            enemy.velocity_y += (dy / distance) * knockback_strength * delta;

            enemy.knockback_cooldown = knockback_duration;
        }
    }
}

pub struct Enemy {
    pub enemy_type: EnemyType,

    pub health: i32,
    pub max_health: i32,
    pub speed: i32,

    pub damage: i32,
    pub time_since_last_attack: f32,
    pub attack_speed: f32,

    pub position: Position,
    pub direction: Direction,
    pub velocity_x: f32,
    pub velocity_y: f32,

    pub knockback_cooldown: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnemyType {
    ServoSkull,
    DarkFighter,
}

impl EnemyType {
    pub fn servo_skull_type() -> EnemyType {
        EnemyType::ServoSkull
    }

    pub fn new_servo_skull(position: Position) -> Enemy {
        Enemy {
            enemy_type: EnemyType::ServoSkull,
            health: 10,
            max_health: 30,
            speed: 450,
            damage: 10,
            time_since_last_attack: 0.0,
            attack_speed: 1.0,
            direction: Direction::Right,
            position,
            velocity_x: 0.0,
            velocity_y: 0.0,
            knockback_cooldown: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Helper function to create a simple player for testing
    fn create_test_player(x: f32, y: f32) -> Player {
        use raylib::ffi;
        use raylib::prelude::*;

        // Create a minimal texture for testing
        let texture = unsafe {
            let raw_texture = ffi::Texture2D {
                id: 0,
                width: 32,
                height: 32,
                mipmaps: 1,
                format: 0,
            };
            Texture2D::from_raw(raw_texture)
        };
        Player::new(Position { x, y }, texture)
    }

    #[test]
    fn test_semi_implicit_euler_movement() {
        // Create a player at (100, 100)
        let player = create_test_player(100.0, 100.0);

        // Create an enemy at (0, 0) with zero initial velocity
        let mut enemy = EnemyType::new_servo_skull(Position { x: 0.0, y: 0.0 });

        let delta = 0.1; // 100ms time step

        println!("Initial state:");
        println!(
            "  Enemy position: ({}, {})",
            enemy.position.x, enemy.position.y
        );
        println!(
            "  Enemy velocity: ({}, {})",
            enemy.velocity_x, enemy.velocity_y
        );

        // Call handle_movement
        handle_movement(&player, &mut enemy, &delta);

        println!("\nAfter first frame:");
        println!(
            "  Enemy position: ({}, {})",
            enemy.position.x, enemy.position.y
        );
        println!(
            "  Enemy velocity: ({}, {})",
            enemy.velocity_x, enemy.velocity_y
        );

        // Verify that the enemy moved toward the player
        assert!(
            enemy.position.x > 0.0,
            "Enemy should move in positive x direction toward player"
        );
        assert!(
            enemy.position.y > 0.0,
            "Enemy should move in positive y direction toward player"
        );

        // Verify that velocity was updated
        assert!(
            enemy.velocity_x > 0.0,
            "Enemy velocity_x should be positive"
        );
        assert!(
            enemy.velocity_y > 0.0,
            "Enemy velocity_y should be positive"
        );

        // Store position after first frame
        let x_after_1 = enemy.position.x;
        let y_after_1 = enemy.position.y;

        // Run another frame
        handle_movement(&player, &mut enemy, &delta);

        println!("\nAfter second frame:");
        println!(
            "  Enemy position: ({}, {})",
            enemy.position.x, enemy.position.y
        );
        println!(
            "  Enemy velocity: ({}, {})",
            enemy.velocity_x, enemy.velocity_y
        );

        // Enemy should continue moving toward player
        assert!(
            enemy.position.x > x_after_1,
            "Enemy should continue moving in positive x"
        );
        assert!(
            enemy.position.y > y_after_1,
            "Enemy should continue moving in positive y"
        );
    }

    #[test]
    fn test_velocity_accumulation() {
        let player = create_test_player(1000.0, 0.0);

        let mut enemy = EnemyType::new_servo_skull(Position { x: 0.0, y: 0.0 });

        let delta = 0.016; // ~60fps

        // Run several frames
        for i in 0..10 {
            handle_movement(&player, &mut enemy, &delta);
            println!(
                "Frame {}: pos=({:.2}, {:.2}), vel=({:.2}, {:.2})",
                i, enemy.position.x, enemy.position.y, enemy.velocity_x, enemy.velocity_y
            );
        }

        // After 10 frames, the enemy should have moved significantly
        assert!(
            enemy.position.x > 5.0,
            "Enemy should have moved significantly after 10 frames (got x={})",
            enemy.position.x
        );

        // Velocity should be non-zero and substantial
        assert!(
            enemy.velocity_x.abs() > 10.0,
            "Enemy should have accumulated substantial velocity (got vx={})",
            enemy.velocity_x
        );
    }

    #[test]
    fn test_friction_effect() {
        // Place player far away so acceleration is minimal
        let player = create_test_player(100.0, 100.0);

        let mut enemy = EnemyType::new_servo_skull(Position { x: 100.0, y: 100.0 });

        // Give enemy initial velocity in x direction (perpendicular to player)
        enemy.velocity_x = 100.0;
        enemy.velocity_y = 0.0;

        let delta = 0.016;

        println!(
            "Initial velocity: ({}, {})",
            enemy.velocity_x, enemy.velocity_y
        );

        let initial_vx = enemy.velocity_x;

        // Run several frames - friction should reduce velocity
        for i in 0..3 {
            let prev_vx = enemy.velocity_x;
            handle_movement(&player, &mut enemy, &delta);
            println!(
                "Frame {}: vel=({:.2}, {:.2})",
                i, enemy.velocity_x, enemy.velocity_y
            );
            // Each frame should reduce velocity
            assert!(
                enemy.velocity_x < prev_vx,
                "Frame {}: Friction should reduce velocity each frame",
                i
            );
        }

        // Velocity should have decreased due to friction
        assert!(
            enemy.velocity_x < initial_vx,
            "Friction should reduce velocity_x from {} to {}",
            initial_vx,
            enemy.velocity_x
        );
    }

    // Helper to create a collision rectangle centered on a position (matches rendering)
    fn create_centered_enemy_rect(pos_x: f32, pos_y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle::new(pos_x - width / 2.0, pos_y - height / 2.0, width, height)
    }

    #[test]
    fn test_enemy_collision_rectangle_centered() {
        // Create an enemy at position (100, 100) with a 64x64 texture
        let enemy_pos_x = 100.0;
        let enemy_pos_y = 100.0;
        let texture_width = 64.0;
        let texture_height = 64.0;

        let rect =
            create_centered_enemy_rect(enemy_pos_x, enemy_pos_y, texture_width, texture_height);

        // The collision rectangle should be centered on enemy position
        // For a 64x64 texture at (100, 100):
        // - Top-left should be at (100 - 32, 100 - 32) = (68, 68)
        // - Bottom-right should be at (68 + 64, 68 + 64) = (132, 132)
        assert_eq!(rect.x, 68.0, "Rectangle X should be position - half_width");
        assert_eq!(rect.y, 68.0, "Rectangle Y should be position - half_height");
        assert_eq!(rect.width, 64.0, "Rectangle width should match texture");
        assert_eq!(rect.height, 64.0, "Rectangle height should match texture");

        // Verify the center of the rectangle is at the enemy position
        let rect_center_x = rect.x + rect.width / 2.0;
        let rect_center_y = rect.y + rect.height / 2.0;
        assert_eq!(
            rect_center_x, enemy_pos_x,
            "Rectangle center X should match enemy position"
        );
        assert_eq!(
            rect_center_y, enemy_pos_y,
            "Rectangle center Y should match enemy position"
        );
    }

    #[test]
    fn test_player_enemy_collision_at_boundary() {
        // Player at origin with known collision radius
        let player = create_test_player(0.0, 0.0);
        let player_radius = player.collision_radius;

        // Enemy texture size (simulated)
        let enemy_half_width = 32.0; // 64x64 texture

        // Place enemy so its left edge just touches player's collision circle
        // Enemy center should be at: player_radius + enemy_half_width
        let boundary_distance = player_radius + enemy_half_width;

        let enemy_rect = create_centered_enemy_rect(boundary_distance, 0.0, 64.0, 64.0);
        let player_point = Vector2::new(0.0, 0.0);

        // At exactly the boundary, collision should occur (edges touching)
        let collides_at_boundary =
            enemy_rect.check_collision_circle_rec(player_point, player_radius);
        assert!(
            collides_at_boundary,
            "Collision should occur when enemy edge touches player circle (distance={})",
            boundary_distance
        );

        // One pixel inside boundary should definitely collide
        let enemy_rect_inside =
            create_centered_enemy_rect(boundary_distance - 1.0, 0.0, 64.0, 64.0);
        let collides_inside =
            enemy_rect_inside.check_collision_circle_rec(player_point, player_radius);
        assert!(
            collides_inside,
            "Collision should occur when enemy is inside boundary"
        );
    }

    #[test]
    fn test_no_collision_when_not_touching() {
        // Player at origin
        let player = create_test_player(0.0, 0.0);
        let player_radius = player.collision_radius;

        // Enemy texture size (simulated)
        let enemy_half_width = 32.0; // 64x64 texture

        // Place enemy well outside collision range
        // Gap of 10 pixels between player circle and enemy rectangle
        let safe_distance = player_radius + enemy_half_width + 10.0;

        let enemy_rect = create_centered_enemy_rect(safe_distance, 0.0, 64.0, 64.0);
        let player_point = Vector2::new(0.0, 0.0);

        let collides = enemy_rect.check_collision_circle_rec(player_point, player_radius);
        assert!(
            !collides,
            "No collision should occur when enemy is {} pixels away (gap of 10px)",
            safe_distance
        );

        // Test diagonal case - enemy far away diagonally
        let enemy_rect_diagonal =
            create_centered_enemy_rect(safe_distance, safe_distance, 64.0, 64.0);
        let collides_diagonal =
            enemy_rect_diagonal.check_collision_circle_rec(player_point, player_radius);
        assert!(
            !collides_diagonal,
            "No collision should occur when enemy is diagonally away"
        );
    }
}
