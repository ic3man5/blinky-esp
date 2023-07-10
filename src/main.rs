use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use esp_idf_hal::{
    gpio::*,
    delay,
    gpio::{IOPin, PinDriver},
    peripherals::{Peripherals, self}, delay::{FreeRtos, Delay, Ets},
};

use dht_sensor::*;

const DHT22_DATA_PIN: u8 = 14;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio2).unwrap();
    let mut dht22_pin = PinDriver::input_output_od(peripherals.pins.gpio14).unwrap();
    
    // Pulling the pin high to avoid confusing the sensor when initalizing
    dht22_pin.set_high().unwrap();
    // The DHT11 datasheet suggests 1 second
    FreeRtos::delay_ms(1000);

    let mut delay = delay::Ets;

    loop {
        match dht22::Reading::read(&mut delay, &mut dht22_pin) {
            Ok(dht22::Reading {
                temperature,
                relative_humidity,
            }) => {
                let f = temperature * 1.8 + 32.0;
                info!("Temperature: {temperature}C ({f}F), Humidity: {relative_humidity}%");
            },
            Err(e) => {
                error!("{e:?}");
            }
        }
        led.set_high().unwrap();
        FreeRtos::delay_ms(500);
        led.set_low().unwrap();
        FreeRtos::delay_ms(500);
    }
}
