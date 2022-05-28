pub struct PanelSetting {
    pub resolution: Resolution,
    pub lut_selection: LutSelection,
    pub colour_selection: ColourSelection,
    pub gate_scan_direction: GateScanDirection,
    pub source_shift_direction: SourceShiftDirection,
    pub booster_enable: BoosterEnable,
    pub soft_reset: SoftReset,
}

impl Default for PanelSetting {
    fn default() -> Self {
        Self {
            resolution: Resolution::Res96x230,
            lut_selection: LutSelection::FromOtp,
            colour_selection: ColourSelection::BlackWhiteRed,
            gate_scan_direction: GateScanDirection::Down,
            source_shift_direction: SourceShiftDirection::Right,
            booster_enable: BoosterEnable::On,
            soft_reset: SoftReset::None,
        }
    }
}

impl From<PanelSetting> for [u8; 1] {
    fn from(ps: PanelSetting) -> [u8; 1] {
        [ps.resolution as u8
            | ps.lut_selection as u8
            | ps.colour_selection as u8
            | ps.gate_scan_direction as u8
            | ps.source_shift_direction as u8
            | ps.booster_enable as u8
            | ps.soft_reset as u8]
    }
}

pub enum Resolution {
    Res96x230 = 0b00_000000,
    Res96x262 = 0b01_000000,
    Res129x296 = 0b10_000000,
    Res160x296 = 0b11_000000,
}

pub enum LutSelection {
    FromOtp = 0b00_0_00000,
    FromRegister = 0b00_1_00000,
}

pub enum ColourSelection {
    BlackWhiteRed = 0b000_0_0000,
    BlackWhite = 0b000_1_0000,
}

pub enum GateScanDirection {
    Up = 0b0000_0_000,
    Down = 0b0000_1_000,
}

pub enum SourceShiftDirection {
    Left = 0b00000_0_00,
    Right = 0b00000_1_00,
}

pub enum BoosterEnable {
    Off = 0b000000_0_0,
    On = 0b000000_1_0,
}

pub enum SoftReset {
    Reset = 0b0000000_0,
    None = 0b0000000_1,
}

// Disabled because tests don't work with no_std
#[cfg(all(test, aaa))]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let default_panel_setting = PanelSetting::default();
        assert_eq!(0x0fu8, default_panel_setting.into())
    }
}
