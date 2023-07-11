use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
};
use esp_idf_hal::{
    delay,
    delay::{Delay, Ets, FreeRtos, BLOCK},
    gpio::*,
    gpio::{IOPin, PinDriver},
    i2c::*,
    i2c::{I2cConfig, I2cDriver},
    peripherals::{self, Peripherals},
    prelude::FromValueType,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use dht_sensor::*;

const DHT22_DATA_PIN: u8 = 14;
const SSD1306_ADDRESS: u8 = 0x3c;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    // Setup DHT22
    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2).unwrap();
    let mut dht22_pin = PinDriver::input_output_od(peripherals.pins.gpio14).unwrap();
    // Pulling the pin high to avoid confusing the sensor when initalizing
    dht22_pin.set_high().unwrap();
    // The DHT11 datasheet suggests 1 second
    FreeRtos::delay_ms(1000);

    // Setup I2C for the OLED Screen (SSD1306)
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio6;
    let scl = peripherals.pins.gpio7;
    let config = I2cConfig::new().baudrate(100_u32.kHz().into());
    //.scl_enable_pullup(true)
    //.sda_enable_pullup(true);
    let i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();
    // Setup the high level driver for the OLED screen (SSD1306)
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // Draw the Rust Logo
    let raw_image: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("images/rust.raw"), 64);
    let image = Image::new(&raw_image, Point::new(32, 0));
    image.draw(&mut display).unwrap();
    display.flush().unwrap();

    loop {
        // Read the temperature
        let mut dht22_delay = delay::Ets;
        match dht22::Reading::read(&mut dht22_delay, &mut dht22_pin) {
            Ok(dht22::Reading {
                temperature,
                relative_humidity,
            }) => {
                let f = temperature * 1.8 + 32.0;
                info!("Temperature: {temperature}C ({f}F), Humidity: {relative_humidity}%");
            }
            Err(e) => {
                error!("{e:?}");
            }
        }
        // Blink the LED
        led.set_high().unwrap();
        FreeRtos::delay_ms(500);
        led.set_low().unwrap();
        FreeRtos::delay_ms(500);
    }
}
