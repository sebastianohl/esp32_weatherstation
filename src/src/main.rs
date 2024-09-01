use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::i2c::{I2c, I2cConfig, I2cDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::units::Hertz;

const SLAVE_ADDR: u8 = 0x76;

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

    //let p = ;
    let peripherals = match Peripherals::take() {
        Ok(p) => p,
        Err(e) => panic!("error getting peripherals {e:?}"),
    };

    let mut i2c_master = match i2c_master_init(
        peripherals.i2c0,
        peripherals.pins.gpio6.into(),
        peripherals.pins.gpio5.into(),
        1000.kHz().into(),
    ) {
        Ok(i2c) => i2c,
        Err(e) => panic!("error i2c init {e:?}"),
    };

    let tx_buf: [u8; 1] = [0xe0];
    let _ = i2c_master.write(SLAVE_ADDR, &tx_buf, BLOCK);
    FreeRtos::delay_ms(5);
    loop {
        let tx_buf: [u8; 1] = [0xd0];
        let _ = i2c_master.write(SLAVE_ADDR, &tx_buf, BLOCK);
        let mut rx_buf: [u8; 1] = [0];
        match i2c_master.read(SLAVE_ADDR, &mut rx_buf, BLOCK) {
            Ok(_) => println!("Master receives {:?}", rx_buf),
            Err(e) => println!("Error: {:?}", e),
        }

        FreeRtos::delay_ms(1000);
    }
}
