#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use as7331_rs::As7331;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::IO, i2c::I2C, peripherals::Peripherals, prelude::*,
};

use log::{error, info};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let delay = Delay::new(&clocks);

    esp_println::logger::init_logger(log::LevelFilter::Info);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c0 = I2C::new(
        peripherals.I2C0,
        io.pins.gpio4,
        io.pins.gpio5,
        400.kHz(),
        &clocks,
        None,
    );
    let mut as7343_sensor = As7343::new(i2c0, as7343::AS7343_I2CADDR_DEFAULT);
    let _ = as7343_sensor.begin();
    let _ = as7343_sensor.set_atime(100);
    let _ = as7343_sensor.set_astep(999);
    let _ = as7343_sensor.set_gain(as7343::AS7343_GAIN_128X);
    let _ = as7343_sensor.enable_led(false, as7343::AS7343_LED_STENGTH_4MA);

    log::info!("looping");
    loop {
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
        FreeRtos::delay_ms(1);
    }
}
