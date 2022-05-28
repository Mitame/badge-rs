pub enum PowerOffSequence {
    Frame1 = 0b00_0000,
    Frame2 = 0b01_0000,
    Frame3 = 0b10_0000,
    Frame4 = 0b11_0000,
}

impl Default for PowerOffSequence {
    fn default() -> Self {
        Self::Frame1
    }
}

impl From<PowerOffSequence> for [u8; 1] {
    fn from(sequence: PowerOffSequence) -> [u8; 1] {
        [sequence as u8]
    }
}
