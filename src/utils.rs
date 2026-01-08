#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Angle(f32), // Angle in radians, 0 = right, PI/2 = down, PI = left, -PI/2 = up
}

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
}
