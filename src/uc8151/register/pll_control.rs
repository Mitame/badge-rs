pub enum PllClockFrequency {
    _29Hz = 0o11,
    _14Hz = 0o12,
    _10Hz = 0o13,
    _7Hz = 0o14,
    _6Hz = 0o15,
    _5Hz = 0o16,
    _4Hz = 0o17,

    _57Hz = 0o21,
    // _29Hz = 0o22,
    _19Hz = 0o23,
    // _14Hz = 0o24,
    _11Hz = 0o25,
    // _10Hz = 0o26,
    _8Hz = 0o27,

    _86Hz = 0o31,
    _43Hz = 0o32,
    // _29Hz = 0o33,
    _21Hz = 0o34,
    _17Hz = 0o35,
    // _14Hz = 0o36,
    _12Hz = 0o37,

    _114Hz = 0o41,
    // _57Hz = 0o42,
    _38Hz = 0o43,
    // _29Hz = 0o44,
    _23Hz = 0o45,
    // _19Hz = 0o46,
    _16Hz = 0o47,

    _150Hz = 0o51,
    _72Hz = 0o52,
    _48Hz = 0o53,
    _36Hz = 0o54,
    // _29Hz = 0o55,
    _24Hz = 0o56,
    _20Hz = 0o57,

    _171Hz = 0o61,
    // _86Hz = 0o62,
    // _57Hz = 0o63,
    // _43Hz = 0o64,
    _34Hz = 0o65,
    // _29Hz = 0o66,
    // _24Hz = 0o67,
    _200Hz = 0o71,
    _100Hz = 0o72,
    _67Hz = 0o73,
    _50Hz = 0o74,
    _40Hz = 0o75,
    _33Hz = 0o76,
    // _29Hz = 0o77,
}

impl Default for PllClockFrequency {
    fn default() -> PllClockFrequency {
        PllClockFrequency::_100Hz
    }
}

impl From<PllClockFrequency> for [u8; 1] {
    fn from(setting: PllClockFrequency) -> [u8; 1] {
        [setting as u8]
    }
}
