use std::vec;

use crate::player::Position;

pub struct AllProjectiles {
    pub projectiles: Vec<Projectile>,
}

impl AllProjectiles {
    pub fn new() -> Self {
        AllProjectiles {
            projectiles: vec![],
        }
    }
    pub fn append(&mut self, new: &mut Vec<Projectile>) {
        self.projectiles.append(new);
    }

    pub fn move_projectiles(&mut self, delta: &f32) {
        for projectile in self.projectiles.iter_mut() {
            match projectile {
                Projectile::Bolter(bolter_data) => match bolter_data.position.direction {
                    crate::player::Direction::Up => {
                        bolter_data.position.y -= bolter_data.speed * delta
                    }
                    crate::player::Direction::Down => {
                        bolter_data.position.y += bolter_data.speed * delta
                    }
                    crate::player::Direction::Left => {
                        bolter_data.position.x -= bolter_data.speed * delta
                    }
                    crate::player::Direction::Right => {
                        bolter_data.position.x += bolter_data.speed * delta
                    }
                },
            };
        }
    }
}

#[derive(Clone, Copy)]
pub enum Projectile {
    Bolter(BolterProjectile),
}

#[derive(Clone, Copy)]
pub struct BolterProjectile {
    pub speed: f32,
    pub position: Position,
}

impl BolterProjectile {
    pub fn new(position: Position) -> Self {
        BolterProjectile {
            speed: 600.0,
            position,
        }
    }
}
