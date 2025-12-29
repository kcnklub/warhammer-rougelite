#[derive(Clone, Copy)]
pub enum Weapon {
    Bolter(BolterData),
}

#[derive(Clone, Copy)]
pub struct BolterData {
    pub damage: f32,
    pub tick_interval: f32,
    pub time_since_last_tick: f32,
}
