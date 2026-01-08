use std::collections::HashMap;

use rand::prelude::*;
use raylib::prelude::*;

use crate::{
    player::Player,
    utils::{Direction, Position},
};

const SPEED: f32 = 2000.0;

pub struct AllEnemies<'a> {
    pub enemies: Vec<Enemy>,
    time_since_spawn: f32,
    spawn_rate: f32,
    pub texture_map: HashMap<EnemyType, &'a Texture2D>,
}

impl<'a> AllEnemies<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        let mut texture_map = HashMap::new();
        texture_map.insert(EnemyType::servo_skull_type(), texture);

        Self {
            enemies: vec![],
            time_since_spawn: 0.0,
            spawn_rate: 3.0,
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

    pub fn spawn_enemies(&mut self, delta: &f32) {
        self.time_since_spawn += delta;
        let mut rng = rand::rng();
        let x: i32 = rng.random_range(1..2480);
        let y: i32 = rng.random_range(1..200);
        if self.time_since_spawn >= self.spawn_rate {
            let spawned_enemy = EnemyType::new_servo_skull(Position {
                x: x as f32,
                y: y as f32,
                direction: Direction::Down,
            });
            self.enemies.push(spawned_enemy);
            self.time_since_spawn = 0.0;
        }
    }
}

fn handle_movement(player: &Player, enemy: &mut Enemy, delta: &f32) {
    // Semi-implicit Euler integration
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
    // (only if distance > 0 to avoid division by zero)
    if distance > 0.0 {
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
        enemy.position.direction = if enemy.velocity_x > 0.0 {
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
    let enemy_rec = Rectangle::new(
        enemy.position.x,
        enemy.position.y,
        texture.width as f32,
        texture.height as f32,
    );
    let player_point = Vector2::new(player.position.x, player.position.y);

    enemy.time_since_last_attack += delta;
    if enemy_rec.check_collision_circle_rec(player_point, (player.texture.width / 2) as f32) {
        if enemy.time_since_last_attack >= enemy.attack_speed {
            player.health -= enemy.damage;
            enemy.time_since_last_attack = 0.0;
        }
        let enemy_center_x = enemy.position.x + (texture.width as f32 / 2.0);
        let enemy_center_y = enemy.position.y + (texture.height as f32 / 2.0);
        let dx = enemy_center_x - player.position.x;
        let dy = enemy_center_y - player.position.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > 0.0 {
            // Apply velocity impulse for knockback (instead of direct position change)
            let knockback_strength = 800.0; // TODO: Tune this value during gameplay testing
            enemy.velocity_x += (dx / distance) * knockback_strength;
            enemy.velocity_y += (dy / distance) * knockback_strength;
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
    pub velocity_x: f32,
    pub velocity_y: f32,
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
            health: 30,
            max_health: 30,
            speed: 450,
            damage: 10,
            time_since_last_attack: 0.0,
            attack_speed: 1.0,
            position,
            velocity_x: 0.0,
            velocity_y: 0.0,
        }
    }

    pub fn dark_fighter_type() -> EnemyType {
        EnemyType::DarkFighter
    }

    pub fn new_dark_fighter(position: Position) -> Enemy {
        Enemy {
            enemy_type: EnemyType::DarkFighter,
            health: 30,
            max_health: 30,
            speed: 450,
            damage: 10,
            time_since_last_attack: 0.0,
            attack_speed: 1.0,
            position,
            velocity_x: 0.0,
            velocity_y: 0.0,
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
        Player {
            position: Position {
                x,
                y,
                direction: Direction::Down,
            },
            move_speed: 300.0,
            health: 100,
            max_health: 100,
            statuses: vec![],
            weapons: vec![],
            texture,
        }
    }

    #[test]
    fn test_semi_implicit_euler_movement() {
        // Create a player at (100, 100)
        let player = create_test_player(100.0, 100.0);

        // Create an enemy at (0, 0) with zero initial velocity
        let mut enemy = EnemyType::new_servo_skull(Position {
            x: 0.0,
            y: 0.0,
            direction: Direction::Down,
        });

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

        let mut enemy = EnemyType::new_servo_skull(Position {
            x: 0.0,
            y: 0.0,
            direction: Direction::Down,
        });

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

        let mut enemy = EnemyType::new_servo_skull(Position {
            x: 100.0,
            y: 100.0,
            direction: Direction::Down,
        });

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
}
