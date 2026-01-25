#[derive(Clone)]
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

    pub fn is_same_type(&self, other: &Weapon) -> bool {
        matches!(
            (self, other),
            (Weapon::Bolter(_), Weapon::Bolter(_))
                | (Weapon::MultiMelta(_), Weapon::MultiMelta(_))
                | (Weapon::PowerSword(_), Weapon::PowerSword(_))
                | (Weapon::Shotgun(_), Weapon::Shotgun(_))
        )
    }

    pub fn increment_stack(&mut self) {
        match self {
            Weapon::Bolter(data)
            | Weapon::MultiMelta(data)
            | Weapon::PowerSword(data)
            | Weapon::Shotgun(data) => {
                data.stack_count += 1;
            }
        }
    }

    pub fn get_stack_count(&self) -> u32 {
        match self {
            Weapon::Bolter(data)
            | Weapon::MultiMelta(data)
            | Weapon::PowerSword(data)
            | Weapon::Shotgun(data) => data.stack_count,
        }
    }
}

#[derive(Clone)]
pub struct WeaponData {
    pub damage: f32,
    pub tick_interval: f32,
    pub time_since_last_tick: f32,
    pub stack_count: u32,
    pub queued_shots: Vec<f32>,
}
