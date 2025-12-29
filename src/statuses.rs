#[derive(Clone, Copy)]
pub enum Status {
    Poison(PoisonStatus),
    Burn(BurnStatus),
    Slow(SlowStatus),
    Stun(StunStatus),
    Regeneration(RegenerationStatus),
    SpeedBoost(SpeedBoostStatus),
}

impl Status {
    pub fn is_expired(&self) -> bool {
        self.get_remaining_duration() <= 0.0
    }

    pub fn get_remaining_duration(&self) -> f32 {
        match self {
            Status::Poison(data) => data.remaining_duration,
            Status::Burn(data) => data.remaining_duration,
            Status::Slow(data) => data.remaining_duration,
            Status::Stun(data) => data.remaining_duration,
            Status::Regeneration(data) => data.remaining_duration,
            Status::SpeedBoost(data) => data.remaining_duration,
        }
    }

    pub fn get_display_name(&self) -> &str {
        match self {
            Status::Poison(_) => "Poison",
            Status::Burn(_) => "Burn",
            Status::Slow(_) => "Slow",
            Status::Stun(_) => "Stun",
            Status::Regeneration(_) => "Regeneration",
            Status::SpeedBoost(_) => "Speed Boost",
        }
    }
}

#[derive(Clone, Copy)]
pub struct PoisonStatus {
    pub damage_per_tick: f32,
    pub tick_interval: f32,
    pub remaining_duration: f32,
    pub time_since_last_tick: f32,
}

#[derive(Clone, Copy)]
pub struct BurnStatus {
    pub damage_per_tick: f32,
    pub tick_interval: f32,
    pub remaining_duration: f32,
    pub time_since_last_tick: f32,
}

#[derive(Clone, Copy)]
pub struct SlowStatus {
    pub speed_multiplier: f32,
    pub remaining_duration: f32,
}

#[derive(Clone, Copy)]
pub struct StunStatus {
    pub remaining_duration: f32,
}

#[derive(Clone, Copy)]
pub struct RegenerationStatus {
    pub heal_per_tick: f32,
    pub tick_interval: f32,
    pub remaining_duration: f32,
    pub time_since_last_tick: f32,
}

#[derive(Clone, Copy)]
pub struct SpeedBoostStatus {
    pub speed_multiplier: f32,
    pub remaining_duration: f32,
}
