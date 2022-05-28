use crate::bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio, pac,
    sio::Sio,
    spi,
    watchdog::Watchdog,
    Spi, Timer,
};
use cortex_m::delay::Delay;
use cortex_m::prelude::*;
use cortex_m_semihosting::*;
use embedded_hal::digital::v2::{InputPin, OutputPin};

mod register;

use self::register::*;

struct WritePins<SclkPinId, MosiPinId>
where
    SclkPinId: gpio::PinId + gpio::bank0::BankPinId,
    MosiPinId: gpio::PinId + gpio::bank0::BankPinId,
{
    sclk: gpio::Pin<SclkPinId, gpio::FunctionSpi>,
    mosi: gpio::Pin<MosiPinId, gpio::FunctionSpi>,
}

impl<SclkPinId, MosiPinId> From<ReadPins<SclkPinId, MosiPinId>> for WritePins<SclkPinId, MosiPinId>
where
    SclkPinId: gpio::PinId + gpio::bank0::BankPinId,
    MosiPinId: gpio::PinId + gpio::bank0::BankPinId,
{
    fn from(pins: ReadPins<SclkPinId, MosiPinId>) -> Self {
        let sclk = pins.sclk.into_mode::<gpio::FunctionSpi>();
        let mosi = pins.mosi.into_mode::<gpio::FunctionSpi>();
        Self { sclk, mosi }
    }
}

struct ReadPins<SclkPinId, MosiPinId>
where
    SclkPinId: gpio::PinId + gpio::bank0::BankPinId,
    MosiPinId: gpio::PinId + gpio::bank0::BankPinId,
{
    sclk: gpio::Pin<SclkPinId, gpio::PushPullOutput>,
    mosi: gpio::Pin<MosiPinId, gpio::Input<gpio::Floating>>,
}

impl<SclkPinId, MosiPinId> From<WritePins<SclkPinId, MosiPinId>> for ReadPins<SclkPinId, MosiPinId>
where
    SclkPinId: gpio::PinId + gpio::bank0::BankPinId,
    MosiPinId: gpio::PinId + gpio::bank0::BankPinId,
{
    fn from(pins: WritePins<SclkPinId, MosiPinId>) -> Self {
        let sclk = pins.sclk.into_push_pull_output();
        let mosi = pins.mosi.into_floating_input();
        Self { sclk, mosi }
    }
}

enum PinMode<SclkPinId, MosiPinId>
where
    SclkPinId: gpio::PinId + gpio::bank0::BankPinId,
    MosiPinId: gpio::PinId + gpio::bank0::BankPinId,
{
    Write(WritePins<SclkPinId, MosiPinId>),
    Read(ReadPins<SclkPinId, MosiPinId>),
}

struct PinContainer<SclkPinId, MosiPinId>(PinMode<SclkPinId, MosiPinId>)
where
    SclkPinId: gpio::PinId + gpio::bank0::BankPinId,
    MosiPinId: gpio::PinId + gpio::bank0::BankPinId;

impl<SclkPinId, MosiPinId> PinContainer<SclkPinId, MosiPinId>
where
    SclkPinId: gpio::PinId + gpio::bank0::BankPinId,
    MosiPinId: gpio::PinId + gpio::bank0::BankPinId,
{
    fn get_write_pins<'a>(&'a mut self) -> &'a mut WritePins<SclkPinId, MosiPinId> {
        use core::mem::{replace, MaybeUninit};
        // This will be replaced back soon, so uninit-ing it for a few cycles shouldn't be a
        // problem.
        let pins = replace(&mut self.0, unsafe { MaybeUninit::uninit().assume_init() });

        self.0 = match pins {
            PinMode::Read(pins) => PinMode::Write(pins.into()),
            PinMode::Write(pins) => PinMode::Write(pins),
        };
        if let PinMode::Write(pins) = &mut self.0 {
            pins
        } else {
            debug::exit(debug::EXIT_SUCCESS);
            loop {}
        }
    }

    fn get_read_pins<'a>(&'a mut self) -> &'a mut ReadPins<SclkPinId, MosiPinId> {
        use core::mem::{replace, MaybeUninit};
        // This will be replaced back soon, so uninit-ing it for a few cycles shouldn't be a
        // problem.
        let pins = replace(&mut self.0, unsafe { MaybeUninit::uninit().assume_init() });

        self.0 = match pins {
            PinMode::Write(pins) => PinMode::Read(pins.into()),
            PinMode::Read(pins) => PinMode::Read(pins),
        };
        if let PinMode::Read(pins) = &mut self.0 {
            pins
        } else {
            debug::exit(debug::EXIT_SUCCESS);
            loop {}
        }
    }
}

pub struct Uc8151<SclkPinId, MosiPinId, DcPinId, CsPinId, BusyPinId, ResetPinId, SpiSpiDevice>
where
    SclkPinId: gpio::PinId + gpio::bank0::BankPinId,
    MosiPinId: gpio::PinId + gpio::bank0::BankPinId,
    DcPinId: gpio::PinId + gpio::bank0::BankPinId,
    CsPinId: gpio::PinId + gpio::bank0::BankPinId,
    BusyPinId: gpio::PinId + gpio::bank0::BankPinId,
    ResetPinId: gpio::PinId + gpio::bank0::BankPinId,
    SpiSpiDevice: spi::SpiDevice,
{
    pins: PinContainer<SclkPinId, MosiPinId>,
    dc_pin: gpio::Pin<DcPinId, gpio::Output<gpio::PushPull>>,
    cs_pin: gpio::Pin<CsPinId, gpio::Output<gpio::PushPull>>,
    busy_pin: gpio::Pin<BusyPinId, gpio::Input<gpio::PullUp>>,
    reset_pin: gpio::Pin<ResetPinId, gpio::Output<gpio::PushPull>>,
    spi: spi::Spi<spi::Enabled, SpiSpiDevice, 8>,
    delay: Delay,
}

impl<SclkPinId, MosiPinId, DcPinId, CsPinId, BusyPinId, ResetPinId, SpiSpiDevice>
    Uc8151<SclkPinId, MosiPinId, DcPinId, CsPinId, BusyPinId, ResetPinId, SpiSpiDevice>
where
    SclkPinId: gpio::PinId + gpio::bank0::BankPinId,
    MosiPinId: gpio::PinId + gpio::bank0::BankPinId,
    DcPinId: gpio::PinId + gpio::bank0::BankPinId,
    CsPinId: gpio::PinId + gpio::bank0::BankPinId,
    BusyPinId: gpio::PinId + gpio::bank0::BankPinId,
    ResetPinId: gpio::PinId + gpio::bank0::BankPinId,
    SpiSpiDevice: spi::SpiDevice,
{
    pub fn new<SclkPinMode, MosiPinMode, DcPinMode, CsPinMode, BusyPinMode, ResetPinMode>(
        sclk_pin: gpio::Pin<SclkPinId, SclkPinMode>,
        mosi_pin: gpio::Pin<MosiPinId, MosiPinMode>,
        dc_pin: gpio::Pin<DcPinId, DcPinMode>,
        cs_pin: gpio::Pin<CsPinId, CsPinMode>,
        busy_pin: gpio::Pin<BusyPinId, BusyPinMode>,
        reset_pin: gpio::Pin<ResetPinId, ResetPinMode>,
        spi: spi::Spi<spi::Enabled, SpiSpiDevice, 8>,
        delay: Delay,
    ) -> Self
    where
        SclkPinMode: gpio::ValidPinMode<SclkPinId> + gpio::PinMode,
        MosiPinMode: gpio::ValidPinMode<MosiPinId> + gpio::PinMode,
        DcPinMode: gpio::ValidPinMode<DcPinId> + gpio::PinMode,
        CsPinMode: gpio::ValidPinMode<CsPinId> + gpio::PinMode,
        BusyPinMode: gpio::ValidPinMode<BusyPinId> + gpio::PinMode,
        ResetPinMode: gpio::ValidPinMode<ResetPinId> + gpio::PinMode,
    {
        // Move pins into write mode...
        let sclk = sclk_pin.into_mode::<gpio::FunctionSpi>();
        let mosi = mosi_pin.into_mode::<gpio::FunctionSpi>();
        let dc = dc_pin.into_push_pull_output();
        let cs = cs_pin.into_push_pull_output();
        let busy = busy_pin.into_pull_up_input();
        let reset = reset_pin.into_push_pull_output();
        Self {
            pins: PinContainer(PinMode::Write(WritePins { sclk, mosi })),
            dc_pin: dc,
            cs_pin: cs,
            busy_pin: busy,
            reset_pin: reset,
            spi,
            delay,
        }
    }

    pub fn busy_wait(&mut self) {
        while self.busy_pin.is_low().unwrap() {}
    }

    pub fn reset(&mut self) {
        self.reset_pin.set_low().unwrap();
        self.delay.delay_ms(10);
        self.reset_pin.set_high().unwrap();
        self.delay.delay_ms(10);
        self.busy_wait();
    }

    pub fn command<'a, I: IntoIterator<Item = &'a u8>>(&mut self, command: u8, data: I) {
        let _pins = self.pins.get_write_pins();
        self.cs_pin.set_low().unwrap();

        // Command mode
        self.dc_pin.set_low().unwrap();
        self.spi.write(&[command]).unwrap();

        let mut data = data.into_iter().peekable();
        if data.peek().is_some() {
            // Data mode
            self.dc_pin.set_high().unwrap();

            use embedded_hal::blocking::spi::WriteIter;
            self.spi.write_iter(data.map(|b| *b)).unwrap();
        }
        self.cs_pin.set_high().unwrap();
    }

    pub fn command_read(&mut self, command: u8, buffer: &mut [u8]) {
        let _pins = self.pins.get_write_pins();
        self.cs_pin.set_low().unwrap();

        // Command mode
        self.dc_pin.set_low().unwrap();
        self.spi.write(&[command]).unwrap();

        // Switch to read mode
        let pins = self.pins.get_read_pins();
        // Switch to data mode
        self.dc_pin.set_high().unwrap();

        // Bit-bang each bit in
        // for(auto i = 0u; i < len; i++) {
        //   int byte = i / 8;
        //   int bit = i % 8;
        //   gpio_put(SCK, true);
        //   bool value = gpio_get(MOSI);
        //   data[byte] |= value << (7-bit);
        //   gpio_put(SCK, false);
        // }
        for i in 0..buffer.len() {
            let byte: usize = i / 8;
            let bit = i % 8;
            pins.sclk.set_high().unwrap();
            let value = pins.mosi.is_high().unwrap();
            let value = match value {
                true => 1 << 7 - bit,
                false => 0,
            };
            buffer[byte] |= value;
            pins.sclk.set_low().unwrap();
        }

        self.cs_pin.set_high().unwrap();
    }

    fn default_luts(&mut self) {
        // LUT_VCOM = 0x20,
        self.command(
            0x20,
            &[
                0x00, 0x64, 0x64, 0x37, 0x00, 0x01, 0x00, 0x8c, 0x8c, 0x00, 0x00, 0x04, 0x00, 0x64,
                0x64, 0x37, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00,
            ],
        );

        // LUT_WW   = 0x21,
        self.command(
            0x21,
            &[
                0x54, 0x64, 0x64, 0x37, 0x00, 0x01, 0x60, 0x8c, 0x8c, 0x00, 0x00, 0x04, 0xa8, 0x64,
                0x64, 0x37, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        );

        // LUT_BW   = 0x22,
        self.command(
            0x22,
            &[
                0x54, 0x64, 0x64, 0x37, 0x00, 0x01, 0x60, 0x8c, 0x8c, 0x00, 0x00, 0x04, 0xa8, 0x64,
                0x64, 0x37, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        );

        // LUT_WB   = 0x23,
        self.command(
            0x23,
            &[
                0xa8, 0x64, 0x64, 0x37, 0x00, 0x01, 0x60, 0x8c, 0x8c, 0x00, 0x00, 0x04, 0x54, 0x64,
                0x64, 0x37, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        );

        // LUT_BB   = 0x24,
        self.command(
            0x24,
            &[
                0xa8, 0x64, 0x64, 0x37, 0x00, 0x01, 0x60, 0x8c, 0x8c, 0x00, 0x00, 0x04, 0x54, 0x64,
                0x64, 0x37, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],
        );

        self.busy_wait();
    }

    pub fn setup(&mut self) {
        self.reset();

        self.panel_setting(PanelSetting {
            resolution: Resolution::Res129x296,
            lut_selection: LutSelection::FromOtp,
            colour_selection: ColourSelection::BlackWhite,
            source_shift_direction: SourceShiftDirection::Right,
            booster_enable: BoosterEnable::On,
            soft_reset: SoftReset::None,
            ..Default::default()
        });

        self.default_luts();

        self.power_setting(PowerSetting {
            source_power: PowerSelection::Internal,
            gate_power: PowerSelection::Internal,
            vcom_voltage: VcomVoltage::Vdh,
            vgh_vgl_voltage: VghVglVoltage::V16,
            internal_vdh_voltage: InternalVoltage::V11_0,
            internal_vdl_voltage: InternalVoltage::V11_0,
            internal_vdhr_voltage: InternalVoltage::V11_0,
        });

        self.power_on(true);

        self.booster_soft_start(BoosterSoftStart {
            phase_a: BoosterPhaseSetting {
                soft_start_period: SoftStartPeriod::_10Ms,
                driving_strength: DrivingStrength::Strength3,
                minimum_off_time: MinimumOffTime::_6_58Us,
            },
            phase_b: BoosterPhaseSetting {
                soft_start_period: SoftStartPeriod::_10Ms,
                driving_strength: DrivingStrength::Strength3,
                minimum_off_time: MinimumOffTime::_6_58Us,
            },
            phase_c: BoosterPhaseSetting {
                soft_start_period: SoftStartPeriod::_10Ms,
                driving_strength: DrivingStrength::Strength3,
                minimum_off_time: MinimumOffTime::_6_58Us,
            },
        });

        self.power_off_sequence_setting(PowerOffSequence::Frame1);
        // TSE
        self.command(0x41, &[0x00]);
        // TCON
        self.command(0x60, &[0x22]);
        // CDI
        self.command(0x50, &[0b01_00_1100]);

        self.pll_control(PllClockFrequency::_100Hz);
        // self.power_off();
    }

    pub fn panel_setting(&mut self, setting: PanelSetting) {
        let setting_bytes: [u8; 1] = setting.into();
        self.command(constant::PANEL_SETTING, &setting_bytes);
    }

    pub fn power_setting(&mut self, setting: PowerSetting) {
        let setting_bytes: [u8; 5] = setting.into();
        self.command(constant::POWER_SETTING, &setting_bytes);
    }

    pub fn power_off(&mut self) {
        self.command(constant::POWER_OFF, &[]);
    }

    pub fn power_off_sequence_setting(&mut self, setting: PowerOffSequence) {
        let setting_bytes: [u8; 1] = setting.into();
        self.command(constant::POWER_OFF_SEQUENCE_SETTINGS, &setting_bytes);
    }

    pub fn power_on(&mut self, blocking: bool) {
        self.command(constant::POWER_ON, &[]);
        if blocking {
            self.busy_wait();
        }
    }

    pub fn power_on_measure(&mut self) {
        self.command(constant::POWER_ON_MEASURE, &[]);
    }

    pub fn booster_soft_start(&mut self, setting: BoosterSoftStart) {
        let setting_bytes: [u8; 3] = setting.into();
        self.command(constant::BOOSTER_SOFT_START, &setting_bytes);
    }

    pub fn deep_sleep(&mut self) {
        self.command(constant::DEEP_SLEEP, &[0xA5]);
    }

    pub fn data_start_transmission_1<'a, I: IntoIterator<Item = &'a u8>>(&mut self, data: I) {
        self.command(constant::DISPLAY_START_TRANSMISSION_1, data)
    }

    pub fn data_stop(&mut self) -> bool {
        let mut buf = [0u8; 1];
        self.command_read(constant::DATA_STOP, &mut buf);
        buf[0] & 0x80 == 1
    }

    pub fn display_refresh(&mut self, blocking: bool) {
        self.command(constant::DISPLAY_REFRESH, &[]);
        if blocking {
            self.busy_wait();
        }
    }

    pub fn data_start_transmission_2<'a, I: IntoIterator<Item = &'a u8>>(&mut self, data: I) {
        self.command(constant::DISPLAY_START_TRANSMISSION_2, data)
    }

    pub fn pll_control(&mut self, setting: PllClockFrequency) {
        let setting_bytes: [u8; 1] = setting.into();
        self.command(constant::PLL_CONTROL, &setting_bytes)
    }

    pub fn temperature_sensor_calibration(&mut self) {}

    pub fn temperature_sensor_enable(&mut self) {}

    pub fn temperature_sensor_write(&mut self) {}

    pub fn temperature_sensor_read(&mut self) {}

    pub fn vcom_and_data_interval_setting(&mut self) {}

    pub fn low_power_detection(&mut self) {}

    pub fn tcon_setting(&mut self) {}

    pub fn resolution_setting(&mut self) {}

    pub fn revision(&mut self) {}

    pub fn get_status(&mut self) {}

    pub fn auto_measure_vcom(&mut self) {}

    pub fn vcom_value(&mut self) {}

    pub fn vcom_dc_setting(&mut self) {}

    pub fn partial_window(&mut self) {}

    pub fn partial_in(&mut self) {}

    pub fn partial_out(&mut self) {}

    pub fn program_mode(&mut self) {}

    pub fn active_program(&mut self) {}

    pub fn read_otp_data(&mut self) {}

    pub fn cascade_setting(&mut self) {}

    pub fn power_saving(&mut self) {}

    pub fn force_temperature(&mut self) {}
}
