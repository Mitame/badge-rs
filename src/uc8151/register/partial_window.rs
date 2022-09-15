pub struct PartialWindow {
    pub horizontal_start_channel_bank: u8,
    pub horizontal_end_channel_bank: u8,
    pub vertical_start_line: u16,
    pub vertical_end_line: u16,
    pub partial_scan: bool,
}

impl Default for PartialWindow {
    fn default() -> Self {
        Self {
            horizontal_start_channel_bank: 0,
            horizontal_end_channel_bank: 0,
            vertical_start_line: 0,
            vertical_end_line: 0,
            partial_scan: false,
        }
    }
}

impl From<PartialWindow> for [u8; 7] {
    fn from(setting: PartialWindow) -> [u8; 7] {
        [
            setting.horizontal_start_channel_bank << 3,
            setting.horizontal_end_channel_bank << 3 | 0x07,
            (setting.vertical_start_line >> 8) as u8 & 0x01,
            (setting.vertical_start_line & 0xff) as u8,
            (setting.vertical_end_line >> 8) as u8 & 0x01,
            (setting.vertical_end_line & 0xff) as u8,
            // match setting.partial_scan {
            //     true => 0x00,
            //     false => 0x01,
            // }
            0b00000001
        ]
    }
}
