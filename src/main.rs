use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

// Gpio uses BCM led_pin numbering. BCM GPIO 23 is tied to physical led_pin 16.
const GPIO_LED: u8 = 23;
const GPIO_SENSOR: u8 = 24;

fn main() -> Result<(), Box<dyn Error>> {
    // Retrieve the GPIO led_pin and configure it as an output.
    let mut led_pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    let sensor_pin = Gpio::new()?.get(GPIO_SENSOR)?.into_input();

    loop {
        led_pin.toggle();
        println!("{}", sensor_pin.read());
        thread::sleep(Duration::from_millis(500));
    }
}