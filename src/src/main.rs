use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::i2c::{I2c, I2cConfig, I2cDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::units::Hertz;

use bme680::*;
use core::time::Duration;

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

    {
        let mut delayer = FreeRtos;
        let mut dev = match Bme680::init(i2c_master, &mut delayer, I2CAddress::Primary) {
            Err(e) => panic!("error i2c init {e:?}"),
            Ok(bme) => bme,
        };

        let settings = SettingsBuilder::new()
            .with_humidity_oversampling(OversamplingSetting::OS2x)
            .with_pressure_oversampling(OversamplingSetting::OS4x)
            .with_temperature_oversampling(OversamplingSetting::OS8x)
            .with_temperature_filter(IIRFilterSize::Size3)
            .with_gas_measurement(Duration::from_millis(1500), 320, 25)
            .with_temperature_offset(-2.2)
            .with_run_gas(true)
            .build();

        if let Err(e) = dev.set_sensor_settings(&mut delayer, settings) {
            panic!("Error: {:?}", e)
        }

        if let Err(e) = dev.set_sensor_mode(&mut delayer, PowerMode::ForcedMode) {
            panic!("Error: {:?}", e)
        }

        loop {
            FreeRtos::delay_ms(1000);
            let power_mode = dev.get_sensor_mode();
            println!("Sensor power mode: {:?}", power_mode);
            println!("Setting forced power modes");

            if let Err(e) = dev.set_sensor_mode(&mut delayer, PowerMode::ForcedMode) {
                panic!("Error: {:?}", e)
            }

            println!("Retrieving sensor data");
            let (data, _state) = match dev.get_sensor_data(&mut delayer) {
                Err(e) => panic!("Error: {:?}", e),
                Ok(data) => data,
            };
            println!("Sensor Data {:?}", data);
            println!("Temperature {}°C", data.temperature_celsius());
            println!("Pressure {}hPa", data.pressure_hpa());
            println!("Humidity {}%", data.humidity_percent());
            println!("Gas Resistence {}Ω", data.gas_resistance_ohm());
        }
    }
}
