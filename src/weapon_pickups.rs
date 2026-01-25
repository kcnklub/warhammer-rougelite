use rand::prelude::*;

use crate::{
    player::Player,
    utils::Position,
    weapons::{Weapon, WeaponData},
};

const PICKUP_RADIUS: f32 = 24.0;
const SPAWN_RADIUS: f32 = 2500.0;
const TARGET_PICKUP_COUNT: usize = 6;

#[derive(Clone)]
pub struct WeaponPickup {
    pub weapon: Weapon,
    pub position: Position,
    pub radius: f32,
}

pub struct AllWeaponPickups {
    pub pickups: Vec<WeaponPickup>,
    spawn_radius: f32,
    target_count: usize,
}

impl AllWeaponPickups {
    pub fn new(player_pos: Position) -> Self {
        let mut pickups = Self {
            pickups: vec![],
            spawn_radius: SPAWN_RADIUS,
            target_count: TARGET_PICKUP_COUNT,
        };
        pickups.spawn_around_player(&player_pos);
        pickups
    }

    pub fn update(&mut self, player: &mut Player) {
        if player.has_full_weapon_slots() {
            self.pickups.clear();
            return;
        }

        self.handle_pickups(player);

        if !player.has_full_weapon_slots() {
            self.spawn_around_player(&player.position);
        } else {
            self.pickups.clear();
        }
    }

    fn handle_pickups(&mut self, player: &mut Player) {
        let mut index = 0;
        while index < self.pickups.len() {
            if player.has_full_weapon_slots() {
                break;
            }

            let pickup = self.pickups[index].clone();
            if is_pickup_in_range(player, &pickup) {
                if player.add_or_stack_weapon(pickup.weapon) {
                    self.pickups.swap_remove(index);
                    continue;
                }
            }

            index += 1;
        }
    }

    fn spawn_around_player(&mut self, player_pos: &Position) {
        let mut rng = rand::rng();
        while self.pickups.len() < self.target_count {
            let position = random_position_within_radius(player_pos, self.spawn_radius, &mut rng);
            let weapon = random_weapon(&mut rng);
            self.pickups.push(WeaponPickup {
                weapon,
                position,
                radius: PICKUP_RADIUS,
            });
        }
    }
}

fn random_position_within_radius(
    player_pos: &Position,
    radius: f32,
    rng: &mut impl Rng,
) -> Position {
    let angle = rng.random_range(0.0..std::f32::consts::TAU);
    let distance = radius * rng.random::<f32>().sqrt();

    Position {
        x: player_pos.x + angle.cos() * distance,
        y: player_pos.y + angle.sin() * distance,
    }
}

fn random_weapon(rng: &mut impl Rng) -> Weapon {
    match rng.random_range(0..4) {
        0 => Weapon::Bolter(WeaponData {
            damage: 10.0,
            tick_interval: 1.0,
            time_since_last_tick: 0.0,
            stack_count: 1,
            queued_shots: vec![],
        }),
        1 => Weapon::PowerSword(WeaponData {
            damage: 24.0,
            tick_interval: 0.6,
            time_since_last_tick: 0.0,
            stack_count: 1,
            queued_shots: vec![],
        }),
        2 => Weapon::Shotgun(WeaponData {
            damage: 8.0,
            tick_interval: 1.2,
            time_since_last_tick: 0.0,
            stack_count: 1,
            queued_shots: vec![],
        }),
        _ => Weapon::MultiMelta(WeaponData {
            damage: 18.0,
            tick_interval: 1.8,
            time_since_last_tick: 0.0,
            stack_count: 1,
            queued_shots: vec![],
        }),
    }
}

fn is_pickup_in_range(player: &Player, pickup: &WeaponPickup) -> bool {
    let dx = player.position.x - pickup.position.x;
    let dy = player.position.y - pickup.position.y;
    let distance_sq = dx * dx + dy * dy;
    let radius = player.collision_radius + pickup.radius;
    distance_sq <= radius * radius
}
