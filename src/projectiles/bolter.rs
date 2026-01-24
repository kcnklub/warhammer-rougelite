use crate::utils::Position;

#[derive(Clone, Copy)]
pub struct BolterProjectile {
    pub speed: f32,
    pub damage: i32,
    pub hits: i32,
    pub position: Position,
    pub angle: f32,
}

impl BolterProjectile {
    pub fn new(position: Position, angle: f32) -> Self {
        BolterProjectile {
            speed: 1000.0,
            damage: 10,
            hits: 0,
            position,
            angle,
        }
    }
}
