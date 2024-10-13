use core::result::Result::{self, Err, Ok};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::sys::EspError;

use log::debug;

pub const AS7343_I2CADDR_DEFAULT: u8 = 0x39;
pub const AS7343_CHIP_ID: u8 = 0x81;

#[allow(unused)]
const AS7343_REG_AUXID: u8 = 0x58;
#[allow(unused)]
const AS7343_REG_REVID: u8 = 0x59;
const AS7343_REG_ID: u8 = 0x5A;
#[allow(unused)]
const AS7343_REG_CFG12: u8 = 0x66;
const AS7343_REG_ENABLE: u8 = 0x80;
const AS7343_REG_ATIME: u8 = 0x81;
#[allow(unused)]
const AS7343_REG_WTIME: u8 = 0x83;
#[allow(unused)]
const AS7343_REG_SP_TH_LOW_LSB: u8 = 0x84;
#[allow(unused)]
const AS7343_REG_SP_TH_LOW_MSB: u8 = 0x85;
#[allow(unused)]
const AS7343_REG_SP_TH_HIGH_LSB: u8 = 0x86;
#[allow(unused)]
const AS7343_REG_SP_TH_HIGH_MSB: u8 = 0x87;
#[allow(unused)]
const AS7343_REG_STATUS: u8 = 0x93;
#[allow(unused)]
const AS7343_REG_ASTATUS: u8 = 0x94;
const AS7343_REG_STATUS_2: u8 = 0x90;
#[allow(unused)]
const AS7343_REG_STATUS_3: u8 = 0x91;
#[allow(unused)]
const AS7343_REG_STATUS_5: u8 = 0xBB;
#[allow(unused)]
const AS7343_REG_STATUS_4: u8 = 0xBC;
const AS7343_REG_CFG0: u8 = 0xBF;
const AS7343_REG_CFG1: u8 = 0xC6;
#[allow(unused)]
const AS7343_REG_CFG3: u8 = 0xC7;
#[allow(unused)]
const AS7343_REG_CFG6: u8 = 0xF5;
#[allow(unused)]
const AS7343_REG_CFG8: u8 = 0xC9;
#[allow(unused)]
const AS7343_REG_CFG9: u8 = 0xCA;
#[allow(unused)]
const AS7343_REG_CFG10: u8 = 0x65;
#[allow(unused)]
const AS7343_REG_PERS: u8 = 0xCF;
#[allow(unused)]
const AS7343_REG_GPIO: u8 = 0x6B;
const AS7343_REG_ASTEP_LOW: u8 = 0xD4;
const AS7343_REG_ASTEP_HIGH: u8 = 0xD5;
const AS7343_REG_CFG20: u8 = 0xD6;
const AS7343_REG_LED: u8 = 0xCD;
#[allow(unused)]
const AS7343_REG_AGC_GAIN_MAX: u8 = 0xD7;
#[allow(unused)]
const AS7343_REG_AZ_CONFIG: u8 = 0xDE;
#[allow(unused)]
const AS7343_REG_FD_TIME_1: u8 = 0xE0;
#[allow(unused)]
const AS7343_REG_FD_TIME_2: u8 = 0xE2;
#[allow(unused)]
const AS7343_REG_FD_CFG0: u8 = 0xDF;
#[allow(unused)]
const AS7343_REG_FD_STATUS: u8 = 0xE3;
#[allow(unused)]
const AS7343_REG_INTENAB: u8 = 0xF9;
#[allow(unused)]
const AS7343_REG_CONTROL: u8 = 0xFA;
#[allow(unused)]
const AS7343_REG_FIFO_MAP: u8 = 0xFC;
#[allow(unused)]
const AS7343_REG_FIFO_LVL: u8 = 0xFD;
#[allow(unused)]
const AS7343_REG_FDATA_LOW: u8 = 0xFE;
#[allow(unused)]
const AS7343_REG_FDATA_HIGH: u8 = 0xFF;

#[allow(unused)]
const AS7343_REG_CH0_DATA_L: u8 = 0x95;
#[allow(unused)]
const AS7343_REG_CH0_DATA_H: u8 = 0x96;
#[allow(unused)]
const AS7343_REG_CH1_DATA_L: u8 = 0x97;
#[allow(unused)]
const AS7343_REG_CH1_DATA_H: u8 = 0x98;
#[allow(unused)]
const AS7343_REG_CH2_DATA_L: u8 = 0x99;
#[allow(unused)]
const AS7343_REG_CH2_DATA_H: u8 = 0x9A;
#[allow(unused)]
const AS7343_REG_CH3_DATA_L: u8 = 0x9B;
#[allow(unused)]
const AS7343_REG_CH3_DATA_H: u8 = 0x9C;
#[allow(unused)]
const AS7343_REG_CH4_DATA_L: u8 = 0x9D;
#[allow(unused)]
const AS7343_REG_CH4_DATA_H: u8 = 0x9E;
#[allow(unused)]
const AS7343_REG_CH5_DATA_L: u8 = 0x9F;
#[allow(unused)]
const AS7343_REG_CH5_DATA_H: u8 = 0xA0;
#[allow(unused)]
const AS7343_REG_CH6_DATA_L: u8 = 0xA1;
#[allow(unused)]
const AS7343_REG_CH6_DATA_H: u8 = 0xA2;
#[allow(unused)]
const AS7343_REG_CH7_DATA_L: u8 = 0xA3;
#[allow(unused)]
const AS7343_REG_CH7_DATA_H: u8 = 0xA4;
#[allow(unused)]
const AS7343_REG_CH8_DATA_L: u8 = 0xA5;
#[allow(unused)]
const AS7343_REG_CH8_DATA_H: u8 = 0xA6;
#[allow(unused)]
const AS7343_REG_CH9_DATA_L: u8 = 0xA7;
#[allow(unused)]
const AS7343_REG_CH9_DATA_H: u8 = 0xA8;
#[allow(unused)]
const AS7343_REG_CH10_DATA_L: u8 = 0xA9;
#[allow(unused)]
const AS7343_REG_CH10_DATA_H: u8 = 0xAA;
#[allow(unused)]
const AS7343_REG_CH11_DATA_L: u8 = 0xAB;
#[allow(unused)]
const AS7343_REG_CH11_DATA_H: u8 = 0xAC;
#[allow(unused)]
const AS7343_REG_CH12_DATA_L: u8 = 0xAD;
#[allow(unused)]
const AS7343_REG_CH12_DATA_H: u8 = 0xAE;
#[allow(unused)]
const AS7343_REG_CH13_DATA_L: u8 = 0xAF;
#[allow(unused)]
const AS7343_REG_CH13_DATA_H: u8 = 0xB0;
#[allow(unused)]
const AS7343_REG_CH14_DATA_L: u8 = 0xB1;
#[allow(unused)]
const AS7343_REG_CH14_DATA_H: u8 = 0xB2;
#[allow(unused)]
const AS7343_REG_CH15_DATA_L: u8 = 0xB3;
#[allow(unused)]
const AS7343_REG_CH15_DATA_H: u8 = 0xB4;
#[allow(unused)]
const AS7343_REG_CH16_DATA_L: u8 = 0xB5;
#[allow(unused)]
const AS7343_REG_CH16_DATA_H: u8 = 0xB6;
#[allow(unused)]
const AS7343_REG_CH17_DATA_L: u8 = 0xB7;
#[allow(unused)]
const AS7343_REG_CH17_DATA_H: u8 = 0xB8;

pub const AS7343_AUTO_SMUX_6CHAN: u8 = 0;
pub const AS7343_AUTO_SMUX_12CHAN: u8 = 1;
pub const AS7343_AUTO_SMUX_18CHAN: u8 = 3;

pub const AS7343_GAIN_0_5X: u8 = 0;
pub const AS7343_GAIN_1X: u8 = 1;
pub const AS7343_GAIN_2X: u8 = 2;
pub const AS7343_GAIN_4X: u8 = 3;
pub const AS7343_GAIN_8X: u8 = 4;
pub const AS7343_GAIN_16X: u8 = 5;
pub const AS7343_GAIN_32X: u8 = 6;
pub const AS7343_GAIN_64X: u8 = 7;
pub const AS7343_GAIN_128X: u8 = 8;
pub const AS7343_GAIN_256X: u8 = 9;
pub const AS7343_GAIN_512X: u8 = 10;
pub const AS7343_GAIN_1024X: u8 = 11;
pub const AS7343_GAIN_2048X: u8 = 12;

pub const AS7343_LED_STENGTH_4MA: u8 = 0;
pub const AS7343_LED_STENGTH_6MA: u8 = 1;
pub const AS7343_LED_STENGTH_8MA: u8 = 2;
pub const AS7343_LED_STENGTH_10MA: u8 = 3;
pub const AS7343_LED_STENGTH_12MA: u8 = 4;

pub const AS7343_CHANNEL_450_FZ: usize = 0;
pub const AS7343_CHANNEL_555_FY: usize = 1;
pub const AS7343_CHANNEL_600_FXL: usize = 2;
pub const AS7343_CHANNEL_855_NIR: usize = 3;
pub const AS7343_CHANNEL_CLEAR_1: usize = 4;
pub const AS7343_CHANNEL_FD_1: usize = 5;
pub const AS7343_CHANNEL_425_F2: usize = 6;
pub const AS7343_CHANNEL_475_F3: usize = 7;
pub const AS7343_CHANNEL_515_F4: usize = 8;
pub const AS7343_CHANNEL_640_F6: usize = 9;
pub const AS7343_CHANNEL_CLEAR_0: usize = 10;
pub const AS7343_CHANNEL_FD_0: usize = 11;
pub const AS7343_CHANNEL_405_F1: usize = 12;
pub const AS7343_CHANNEL_550_F5: usize = 13;
pub const AS7343_CHANNEL_690_F7: usize = 14;
pub const AS7343_CHANNEL_745_F8: usize = 15;
pub const AS7343_CHANNEL_CLEAR: usize = 16;
pub const AS7343_CHANNEL_FD: usize = 17;

pub struct As7343<'a> {
    pub i2c: I2cDriver<'a>,
    pub addr: u8,
}

#[allow(dead_code)]
impl<'a> As7343<'a> {
    pub fn new(i2c: I2cDriver<'a>, addr: u8) -> Self {
        As7343 { i2c, addr }
    }

    pub fn destroy(self) -> I2cDriver<'a> {
        self.i2c
    }

    pub fn begin(&mut self) -> Result<bool, EspError> {
        match self.get_chip_id() {
            Err(e) => return Err(e),
            Ok(id) => {
                match id {
                    AS7343_CHIP_ID => {
                        log::info!("as7343 chip found");
                        self.power_enable(true)?;
                        self.set_auto_smux(AS7343_AUTO_SMUX_18CHAN)?;
                        return Ok(true);
                    }
                    _ => {
                        log::info!("as7343 chip id incorrect: {}", id);
                        return Ok(false);
                    }
                };
            }
        };
    }

    fn set_bank(&mut self, low: bool) -> Result<(), EspError> {
        let mut reg = [0u8; 1];
        self.i2c_read_bytes(AS7343_REG_CFG0, &mut reg)?;
        return self.i2c_write_cmd(
            AS7343_REG_CFG0,
            (reg[0] & 0xef) | (if low { 0x10u8 } else { 0x0u8 }),
        );
    }

    pub fn get_chip_id(&mut self) -> Result<u8, EspError> {
        self.set_bank(true)?;
        let mut data = [0u8; 1];
        self.i2c_write_read_cmd(AS7343_REG_ID, &mut data)?;
        self.set_bank(false)?;
        Ok(data[0])
    }

    pub fn power_enable(&mut self, power: bool) -> Result<(), EspError> {
        let mut reg = [0u8; 1];
        self.i2c_read_bytes(AS7343_REG_ENABLE, &mut reg)?;
        return self.i2c_write_cmd(
            AS7343_REG_ENABLE,
            (reg[0] & 0xfe) | (if power { 0x1u8 } else { 0x0u8 }),
        );
    }

    pub fn set_auto_smux(&mut self, smux: u8) -> Result<(), EspError> {
        let mut reg = [0u8; 1];
        self.i2c_read_bytes(AS7343_REG_CFG20, &mut reg)?;
        return self.i2c_write_cmd(AS7343_REG_CFG20, (reg[0] & 0x8f) | ((smux & 0x3) << 5));
    }

    //  Total integration time will be `(ATIME + 1) * (ASTEP + 1) * 2.78µS`
    pub fn set_atime(&mut self, atime: u8) -> Result<(), EspError> {
        return self.i2c_write_cmd(AS7343_REG_ATIME, atime);
    }
    //  Total integration time will be `(ATIME + 1) * (ASTEP + 1) * 2.78µS`
    pub fn set_astep(&mut self, step: u16 /* in 2.78uS */) -> Result<(), EspError> {
        self.i2c_write_cmd(AS7343_REG_ASTEP_LOW, (step & 0xff).try_into().unwrap())?;
        return self.i2c_write_cmd(
            AS7343_REG_ASTEP_HIGH,
            ((step & 0xff00) >> 8).try_into().unwrap(),
        );
    }
    //  Total integration time will be `(ATIME + 1) * (ASTEP + 1) * 2.78µS`
    pub fn get_atime(&mut self) -> Result<u8, EspError> {
        let mut reg = [0u8; 1];
        self.i2c_read_bytes(AS7343_REG_ATIME, &mut reg)?;
        return Ok(reg[0]);
    }
    //  Total integration time will be `(ATIME + 1) * (ASTEP + 1) * 2.78µS`
    pub fn get_astep(&mut self) -> Result<u16, EspError> {
        let mut reg = [0u8; 1];

        self.i2c_read_bytes(AS7343_REG_ASTEP_LOW, &mut reg)?;
        let astep_low = reg[0] as u16;
        self.i2c_read_bytes(AS7343_REG_ASTEP_HIGH, &mut reg)?;
        return Ok(((reg[0] as u16) << 8) & astep_low);
    }
    pub fn set_gain(&mut self, gain: u8) -> Result<(), EspError> {
        let mut reg = [0u8; 1];
        self.i2c_read_bytes(AS7343_REG_CFG1, &mut reg)?;
        return self.i2c_write_cmd(AS7343_REG_CFG1, (reg[0] & 0xf8) | (gain & 0x7));
    }
    pub fn get_gain(&mut self) -> Result<u8, EspError> {
        let mut reg = [0u8; 1];
        match self.i2c_read_bytes(AS7343_REG_CFG1, &mut reg) {
            Err(e) => return Err(e),
            Ok(_) => {
                return Ok(reg[0] & 0x7);
            }
        };
    }
    pub fn enable_led(&mut self, enabled: bool, strength: u8) -> Result<(), EspError> {
        self.set_bank(true)?;
        self.i2c_write_cmd(
            AS7343_REG_LED,
            (if enabled { 0xf0 } else { 0x0 }) | (strength & 0x7f),
        )?;
        self.set_bank(false)?;
        Ok(())
    }

    pub fn clear_digital_saturation_status(&mut self) -> Result<(), EspError> {
        let mut reg = [0u8; 1];
        self.i2c_read_bytes(AS7343_REG_STATUS_2, &mut reg)?;
        return self.i2c_write_cmd(AS7343_REG_STATUS_2, reg[0] & 0xef);
    }
    pub fn clear_analog_saturation_status(&mut self) -> Result<(), EspError> {
        let mut reg = [0u8; 1];
        self.i2c_read_bytes(AS7343_REG_STATUS_2, &mut reg)?;
        return self.i2c_write_cmd(AS7343_REG_STATUS_2, reg[0] & 0xf7);
    }

    pub fn enable_spectral_measurement(&mut self, enabled: bool) -> Result<(), EspError> {
        let mut reg = [0u8; 1];
        self.i2c_read_bytes(AS7343_REG_ENABLE, &mut reg)?;
        return self.i2c_write_cmd(
            AS7343_REG_ENABLE,
            (reg[0] & 0xfd) | (if enabled { 0x2u8 } else { 0x0u8 }),
        );
    }

    pub fn is_data_ready(&mut self) -> Result<bool, EspError> {
        let mut reg = [0u8; 1];
        self.i2c_read_bytes(AS7343_REG_STATUS_2, &mut reg)?;
        Ok(reg[0] & 0x40 > 0)
    }

    pub fn get_digital_saturation(&mut self) -> Result<bool, EspError> {
        let mut reg = [0u8; 1];
        self.i2c_read_bytes(AS7343_REG_STATUS_2, &mut reg)?;
        Ok(reg[0] & 0x10 > 0)
    }

    pub fn get_analog_saturation(&mut self) -> Result<bool, EspError> {
        let mut reg = [0u8; 1];
        self.i2c_read_bytes(AS7343_REG_STATUS_2, &mut reg)?;
        Ok(reg[0] & 0x08 > 0)
    }

    pub fn wait_for_data(&mut self, wait_time: u32) -> Result<bool, EspError> {
        if wait_time == 0 {
            loop {
                match self.is_data_ready() {
                    Err(e) => return Err(e),
                    Ok(b) => {
                        if b {
                            return Ok(true);
                        } else {
                        }
                    }
                };
                FreeRtos::delay_ms(1);
            }
        } else if wait_time > 0 {
            let count = 0;
            loop {
                match self.is_data_ready() {
                    Err(e) => return Err(e),
                    Ok(b) => {
                        if b {
                            return Ok(true);
                        } else {
                        }
                    }
                };
                FreeRtos::delay_ms(1);
                let count = count + 1;
                if count > wait_time {
                    break;
                } else {
                }
            }
            return Ok(false);
        } else {
            return Ok(false);
        }
    }

    fn read_channel(&mut self, channel: u8) -> Result<u16, EspError> {
        let mut reg = [0u8; 1];

        self.i2c_read_bytes(AS7343_REG_CH0_DATA_L + (2 * channel), &mut reg)?;
        let low_byte = reg[0] as u16;
        self.i2c_read_bytes(AS7343_REG_CH0_DATA_L + (2 * channel) + 1, &mut reg)?;
        return Ok(((reg[0] as u16) << 8) & low_byte);
    }

    pub fn read_all_channels(&mut self) -> Result<[u16; 18], EspError> {
        self.enable_spectral_measurement(true)?;
        self.wait_for_data(0)?; // I'll wait for you for all time
        let mut reg = [0u8; 36];
        self.i2c_read_bytes(AS7343_REG_CH0_DATA_L, &mut reg)?;
        return Ok([
            ((reg[1] as u16) << 8) + reg[0] as u16,
            ((reg[3] as u16) << 8) + reg[2] as u16,
            ((reg[5] as u16) << 8) + reg[4] as u16,
            ((reg[7] as u16) << 8) + reg[6] as u16,
            ((reg[9] as u16) << 8) + reg[8] as u16,
            ((reg[11] as u16) << 8) + reg[10] as u16,
            ((reg[13] as u16) << 8) + reg[12] as u16,
            ((reg[15] as u16) << 8) + reg[14] as u16,
            ((reg[17] as u16) << 8) + reg[16] as u16,
            ((reg[19] as u16) << 8) + reg[18] as u16,
            ((reg[21] as u16) << 8) + reg[20] as u16,
            ((reg[23] as u16) << 8) + reg[22] as u16,
            ((reg[25] as u16) << 8) + reg[24] as u16,
            ((reg[27] as u16) << 8) + reg[26] as u16,
            ((reg[29] as u16) << 8) + reg[28] as u16,
            ((reg[31] as u16) << 8) + reg[30] as u16,
            ((reg[33] as u16) << 8) + reg[32] as u16,
            ((reg[35] as u16) << 8) + reg[34] as u16,
        ]);
    }

    fn raw_to_basic_counts(&mut self, raw: u16) -> Result<f32, EspError> {
        let gain_val: f32 = match self.get_gain()? {
            AS7343_GAIN_0_5X => 0.5,
            g => (1 << (g - 1)) as f32,
        };
        let atime = self.get_atime()?;
        let astep = self.get_astep()?;
        return Ok((raw as f32)
            / (gain_val * (atime + 1) as f32 * (astep + 1) as f32 * 2.78f32 / 1000.0f32));
    }

    fn calibration_to_basic_counts(&self, count: u16, time_ms: f32, gain: u8) -> f32 {
        let gain_val: f32 = match gain {
            AS7343_GAIN_0_5X => 0.5,
            g => (1 << (g - 1)) as f32,
        };
        return (count as f32) / (gain_val * time_ms);
    }

    // read calibration data from data sheet (v6-00, 2023-06-07) Section 6, Figure 8
    pub fn raw_to_uwm2(
        &mut self,
        reading_raw: u16,
        calibration_count: u16,
        calibration_time_ms: f32,
        calibration_gain: u8,
        calibration_ee: f32,
    ) -> Result<f32, EspError> {
        let calibration_bc = self.calibration_to_basic_counts(
            calibration_count,
            calibration_time_ms,
            calibration_gain,
        );
        let reading_bc = self.raw_to_basic_counts(reading_raw)?;
        return Ok(reading_bc / calibration_bc * calibration_ee);
    }

    fn i2c_write_read_cmd(&mut self, addr: u8, data: &mut [u8]) -> Result<(), EspError> {
        match self.i2c.write_read(self.addr, &[addr], data, BLOCK) {
            Ok(_) => debug!(
                "I2C_WRITE_READ - ADDR: 0x{:02X} - READ: 0x{:02X}",
                addr, data[0]
            ),
            Err(e) => return Err(e),
        }
        Ok(())
    }

    fn i2c_read_bytes(&mut self, addr: u8, data: &mut [u8]) -> Result<(), EspError> {
        match self.i2c.write_read(self.addr, &[addr], data, BLOCK) {
            Ok(_) => debug!("I2C_READ_BYTES - ADDR: 0x{:02X} - DATA {:?}", addr, data),
            Err(e) => return Err(e),
        }
        Ok(())
    }

    fn i2c_write_cmd(&mut self, addr: u8, cmd: u8) -> Result<(), EspError> {
        match self.i2c.write(self.addr, &[addr, cmd], BLOCK) {
            Ok(_) => debug!("I2C_WRITE - ADDR: 0x{:02X} - DATa: 0x{:02X}", addr, cmd),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}
