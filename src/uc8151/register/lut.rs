#[derive(Copy, Clone)]
pub enum LevelSelection {
    Gnd = 0b00,
    Vdh = 0b01,
    Vdl = 0b10,
    Vdhr = 0b11,
}

#[derive(Clone)]
pub struct LutSetting {
    pub level_select_1: LevelSelection,
    pub level_select_2: LevelSelection,
    pub level_select_3: LevelSelection,
    pub level_select_4: LevelSelection,
    pub number_of_frames_1: u8,
    pub number_of_frames_2: u8,
    pub number_of_frames_3: u8,
    pub number_of_frames_4: u8,
    pub times_to_repeat: u8,
}

impl Default for LutSetting {
    fn default() -> Self {
        Self {
            level_select_1: LevelSelection::Gnd,
            level_select_2: LevelSelection::Gnd,
            level_select_3: LevelSelection::Gnd,
            level_select_4: LevelSelection::Gnd,
            number_of_frames_1: 0,
            number_of_frames_2: 0,
            number_of_frames_3: 0,
            number_of_frames_4: 0,
            times_to_repeat: 0,
        }
    }
}

impl From<&LutSetting> for [u8; 6] {
    fn from(setting: &LutSetting) -> [u8; 6] {
        [
            (setting.level_select_1 as u8) << 6 | (setting.level_select_1 as u8) << 4 | (setting.level_select_1 as u8) << 2 | (setting.level_select_1 as u8),
            setting.number_of_frames_1,
            setting.number_of_frames_2,
            setting.number_of_frames_3,
            setting.number_of_frames_4,
            setting.times_to_repeat
        ]
    }
}

#[derive(Default, Clone)]
pub struct LutSettingGroup(pub [LutSetting; 7]);

impl From<LutSettingGroup> for [u8; 49] {
    fn from(group: LutSettingGroup) -> [u8; 49] {
        let mut bytes = [0u8; 49];
        for (i, setting) in group.0.iter().enumerate() {
            let setting_bytes: [u8; 6] = setting.into();
            bytes[i*6..(i+1)*6].copy_from_slice(&setting_bytes);
        }

        bytes
    }
}
