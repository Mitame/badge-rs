#[derive(Default)]
pub struct BoosterSoftStart {
    pub phase_a: BoosterPhaseSetting,
    pub phase_b: BoosterPhaseSetting,
    pub phase_c: BoosterPhaseSetting,
}

impl From<BoosterSoftStart> for [u8; 3] {
    fn from(settings: BoosterSoftStart) -> [u8; 3] {
        [
            settings.phase_a.into(),
            settings.phase_b.into(),
            settings.phase_c.into(),
        ]
    }
}

pub struct BoosterPhaseSetting {
    pub soft_start_period: SoftStartPeriod,
    pub driving_strength: DrivingStrength,
    pub minimum_off_time: MinimumOffTime,
}

impl From<BoosterPhaseSetting> for u8 {
    fn from(settings: BoosterPhaseSetting) -> u8 {
        settings.soft_start_period as u8
            | settings.driving_strength as u8
            | settings.minimum_off_time as u8
    }
}

impl Default for BoosterPhaseSetting {
    fn default() -> Self {
        Self {
            soft_start_period: SoftStartPeriod::_10Ms,
            driving_strength: DrivingStrength::Strength3,
            minimum_off_time: MinimumOffTime::_6_58Us,
        }
    }
}

pub enum SoftStartPeriod {
    _10Ms = 0o000,
    _20Ms = 0o100,
    _30Ms = 0o200,
    _40Ms = 0o300,
}

pub enum DrivingStrength {
    Strength1 = 0o000,
    Strength2 = 0o010,
    Strength3 = 0o020,
    Strength4 = 0o030,
    Strength5 = 0o040,
    Strength6 = 0o050,
    Strength7 = 0o060,
    Strength8 = 0o070,
}

pub enum MinimumOffTime {
    _0_27Us = 0o000,
    _0_34Us = 0o001,
    _0_40Us = 0o002,
    _0_54Us = 0o003,
    _0_80Us = 0o004,
    _1_54Us = 0o005,
    _3_34Us = 0o006,
    _6_58Us = 0o007,
}
