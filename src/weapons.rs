#[derive(Clone, Copy)]
pub enum Weapon {
    Bolter(WeaponData),
    PowerSword(WeaponData),
}

#[derive(Clone, Copy)]
pub struct WeaponData {
    pub damage: f32,
    pub tick_interval: f32,
    pub time_since_last_tick: f32,
}
