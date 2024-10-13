use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::i2c::{I2c, I2cConfig, I2cDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::units::Hertz;

use core::cell::RefCell;
use embedded_hal_bus::i2c;

use bosch_bme680::Bme680;
use bosch_bme680::DeviceAddress;
use esp_idf_hal::delay::Ets;

use as7331_rs::*;
use as7343_rs::*;

fn i2c_master_init<'d>(
    i2c: impl Peripheral<P = impl I2c> + 'd,
    sda: AnyIOPin,
    scl: AnyIOPin,
    baudrate: Hertz,
) -> anyhow::Result<I2cDriver<'d>> {
    let config = I2cConfig::new().baudrate(baudrate);
    let driver = I2cDriver::new(i2c, sda, scl, &config)?;
    Ok(driver)
}

fn read_as7331<I2C>(as7331_sensor: &mut as7331_rs::As7331<I2C>)
where
    I2C: embedded_hal::i2c::I2c,
{
    let lsb_a = 304.69 / ((1 << (11 - 8)) as f32) / ((1 << 9) as f32 / 1024.0) / 1000.0;
    let lsb_b = 398.44 / ((1 << (11 - 8)) as f32) / ((1 << 9) as f32 / 1024.0) / 1000.0;
    let lsb_c = 191.41 / ((1 << (11 - 8)) as f32) / ((1 << 9) as f32 / 1024.0) / 1000.0;
    let status = as7331_sensor.get_status().unwrap();
    //log::info!("status {:?}", status);
    if status[3] == 1 {
        let all_data = as7331_sensor.read_all_data().unwrap();
        let temp = all_data[0];
        let uv_a = all_data[1];
        let uv_b = all_data[2];
        let uv_c = all_data[3];

        log::info!("AS7331 UV DATA:");
        log::info!("AS7331 UVA: {:.2} (uW/cm^2)", uv_a as f32 * lsb_a);
        log::info!("AS7331 UVB: {:.2} (uW/cm^2)", uv_b as f32 * lsb_b);
        log::info!("AS7331 UVC: {:.2} (uW/cm^2)", uv_c as f32 * lsb_c);
        log::info!(
            "AS7331 Temperature: {:.2} (Celcius)",
            temp as f32 * 0.05 - 66.9
        );
    }
}

fn read_as7341<I2C>(as7343_sensor: &mut as7343_rs::As7343<I2C>)
where
    I2C: embedded_hal::i2c::I2c,
{
    let _ = as7343_sensor.clear_digital_saturation_status();
    let _ = as7343_sensor.clear_analog_saturation_status();
    match as7343_sensor.read_all_channels() {
        Err(e) => panic!("error read all channels {e:?}"),
        Ok(channels) => {
            log::info!(
                "F1 405nm  : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_405_F1],
                    5749,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "F2 425nm  : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_425_F2],
                    1756,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );

            log::info!(
                "FZ 450nm  : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_450_FZ],
                    2169,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "F3 475nm  : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_475_F3],
                    770,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "F4 515nm  : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_515_F4],
                    3141,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "F5 550nm  : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_550_F5],
                    1574,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "FY 555nm  : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_555_FY],
                    3747,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "FXL 600nm : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_600_FXL],
                    4776,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "F6 640nm  : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_640_F6],
                    3336,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "F7 690nm  : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_690_F7],
                    5435,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "F8 745nm  : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_745_F8],
                    864,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "NIR 855nm : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_855_NIR],
                    10581,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "Clear     : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_CLEAR],
                    4311,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_64,
                    155.0
                )
            );
            log::info!(
                "Clear 0   : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_CLEAR_0],
                    999,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "Clear 1   : {:?}",
                as7343_sensor.raw_to_uwm2(
                    channels[as7343::AS7343_CHANNEL_CLEAR_1],
                    999,
                    27.8,
                    as7331::AS7331_CREG1_GAIN_1024,
                    155.0
                )
            );
            log::info!(
                "Digital saturation : {}",
                match as7343_sensor.get_digital_saturation() {
                    Err(e) => panic!("error get digital saturation {e:?}"),
                    Ok(b) => b,
                }
            );
            log::info!(
                "Analog saturation  : {}",
                match as7343_sensor.get_analog_saturation() {
                    Err(e) => panic!("error get digital saturation {e:?}"),
                    Ok(b) => b,
                }
            );
        }
    }
}

fn read_bme680<I2C, D>(bme: &mut Bme680<I2C, D>)
where
    I2C: embedded_hal::i2c::I2c,
    D: embedded_hal::delay::DelayNs,
{
    let values = bme.measure().unwrap();
    log::info!("Values: {values:?}");
}

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello!!!!!");

    let peripherals = match Peripherals::take() {
        Ok(p) => p,
        Err(e) => panic!("error getting peripherals {e:?}"),
    };

    let i2c_master = match i2c_master_init::<'_>(
        peripherals.i2c0,
        peripherals.pins.gpio6.into(),
        peripherals.pins.gpio5.into(),
        100.kHz().into(),
    ) {
        Ok(i2c) => i2c,
        Err(e) => panic!("error i2c init {e:?}"),
    };
    let i2c_ref_cell = RefCell::new(i2c_master);

    let mut as7343_sensor = As7343::new(
        i2c::RefCellDevice::new(&i2c_ref_cell),
        as7343::AS7343_I2CADDR_DEFAULT,
    );
    let _ = as7343_sensor.begin();
    let _ = as7343_sensor.set_atime(100);
    let _ = as7343_sensor.set_astep(999);
    let _ = as7343_sensor.set_gain(as7343::AS7343_GAIN_128X);
    let _ = as7343_sensor.enable_led(false, as7343::AS7343_LED_STENGTH_4MA);

    let mut as7331_sensor = As7331::new(
        i2c::RefCellDevice::new(&i2c_ref_cell),
        as7331::AS7331_I2CADDR_DEFAULT,
    );

    let _ = as7331_sensor.power_up();
    let _ = as7331_sensor.reset();

    FreeRtos::delay_ms(100);
    let chip_id = as7331_sensor.get_chip_id().unwrap();

    if chip_id == 0x21 {
        log::info!("state {:?}", as7331_sensor.get_mode());
        let _ = as7331_sensor.set_configuration_mode();
        let _ = as7331_sensor.init(
            as7331::AS7331_CREG3_MMODE_CONT,
            as7331::AS7331_CREG3_CCLK_1024,
            as7331::AS7331_CREG3_SB_ON,
            255,
            as7331::AS7331_CREG1_GAIN_8,
            as7331::AS7331_CREG1_TIME_512,
        );
        FreeRtos::delay_ms(100);
        let _ = as7331_sensor.set_measurement_mode();
        log::info!("state {:?}", as7331_sensor.get_mode());
    } else {
        panic!("Wrong chip id: {}", chip_id);
    }

    let config = bosch_bme680::Configuration::default();
    // Ets {} is used to create short delays between communication with the sensor.
    // The last parameter is the initial ambient temperature for humidity and pressure calculation.
    // It will be updated automatically with the measured temperature after the first measurment.
    let mut bme = Bme680::new(
        i2c::RefCellDevice::new(&i2c_ref_cell),
        DeviceAddress::Primary,
        Ets {},
        &config,
        20,
    )
    .unwrap();
    FreeRtos::delay_ms(1000);

    loop {
        FreeRtos::delay_ms(1000);
        read_bme680(&mut bme);
        read_as7341(&mut as7343_sensor);
        read_as7331(&mut as7331_sensor);
    }
}
