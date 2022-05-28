pub struct PowerSetting {
    pub source_power: PowerSelection,
    pub gate_power: PowerSelection,
    pub vcom_voltage: VcomVoltage,
    pub vgh_vgl_voltage: VghVglVoltage,
    pub internal_vdh_voltage: InternalVoltage,
    pub internal_vdl_voltage: InternalVoltage,
    pub internal_vdhr_voltage: InternalVoltage,
}

impl Default for PowerSetting {
    fn default() -> Self {
        Self {
            source_power: PowerSelection::Internal,
            gate_power: PowerSelection::Internal,
            vcom_voltage: VcomVoltage::Vdh,
            vgh_vgl_voltage: VghVglVoltage::V16,
            internal_vdh_voltage: InternalVoltage::V10_0,
            internal_vdl_voltage: InternalVoltage::V10_0,
            internal_vdhr_voltage: InternalVoltage::V3_0,
        }
    }
}

impl From<PowerSetting> for [u8; 5] {
    fn from(setting: PowerSetting) -> [u8; 5] {
        [
            (setting.source_power as u8) << 1 | setting.gate_power as u8,
            setting.vcom_voltage as u8 | setting.vgh_vgl_voltage as u8,
            setting.internal_vdh_voltage as u8,
            setting.internal_vdl_voltage as u8,
            setting.internal_vdhr_voltage as u8,
        ]
    }
}

pub enum PowerSelection {
    External = 0,
    Internal = 1,
}

pub enum VcomVoltage {
    /// VCOMH=VDH+DC-VCOM
    Vdh = 0b00000_0_00,
    /// VCOMH=VGH
    Vgh = 0b00000_1_00,
}

pub enum VghVglVoltage {
    /// VGH=16V, VGL= -16V
    V16 = 0b000000_00,
    V15 = 0b000000_01,
    V14 = 0b000000_10,
    V13 = 0b000000_11,
}

pub enum InternalVoltage {
    V2_4 = 0b000000,
    V2_6 = 0b000001,
    V2_8 = 0b000010,

    V3_0 = 0b000011,
    V3_2 = 0b000100,
    V3_4 = 0b000101,
    V3_6 = 0b000110,
    V3_8 = 0b000111,

    V4_0 = 0b001000,
    V4_2 = 0b001001,
    V4_4 = 0b001010,
    V4_6 = 0b001011,
    V4_8 = 0b001100,

    V5_0 = 0b001101,
    V5_2 = 0b001110,
    V5_4 = 0b001111,
    V5_6 = 0b010000,
    V5_8 = 0b010001,

    V6_0 = 0b010010,
    V6_2 = 0b010011,
    V6_4 = 0b010100,
    V6_6 = 0b010101,
    V6_8 = 0b010110,

    V7_0 = 0b010111,
    V7_2 = 0b011000,
    V7_4 = 0b011001,
    V7_6 = 0b011010,
    V7_8 = 0b011011,

    V8_0 = 0b011100,
    V8_2 = 0b011101,
    V8_4 = 0b011110,
    V8_6 = 0b011111,
    V8_8 = 0b100000,

    V9_0 = 0b100001,
    V9_2 = 0b100010,
    V9_4 = 0b100011,
    V9_6 = 0b100100,
    V9_8 = 0b100101,

    V10_0 = 0b100110,
    V10_2 = 0b100111,
    V10_4 = 0b101000,
    V10_6 = 0b101001,
    V10_8 = 0b101010,

    V11_0 = 0b101011,
}
