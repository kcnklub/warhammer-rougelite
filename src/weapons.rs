#[derive(Clone, Copy)]
pub enum Weapon {
    Bolter(WeaponData),
    MultiMelta(WeaponData),
    PowerSword(WeaponData),
    Shotgun(WeaponData),
}

impl Weapon {
    pub fn get_display_name(&self) -> &str {
        match self {
            Weapon::Bolter(_) => "Bolter",
            Weapon::MultiMelta(_) => "Multi Melta",
            Weapon::PowerSword(_) => "Power Sword",
            Weapon::Shotgun(_) => "Shotgun",
        }
    }
}

#[derive(Clone, Copy)]
pub struct WeaponData {
    pub damage: f32,
    pub tick_interval: f32,
    pub time_since_last_tick: f32,
}
