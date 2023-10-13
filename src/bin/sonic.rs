use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use rppal::gpio::Gpio;
fn main() -> ! {
    let gpio = Gpio::new().unwrap();
    let mut trigger = gpio.get(22).unwrap().into_output();
    let echo = gpio.get(10).unwrap().into_input();

    trigger.set_low();
    loop {
        trigger.set_high();
        sleep(Duration::from_micros(10));
        trigger.set_low();
        let start = Instant::now();
        while echo.is_low() {}
        while echo.is_high() {}
        let end = Instant::now();
        println!("{}mm", (end - start).as_micros() as f64 / 58.0);
        sleep(Duration::from_millis(100));
    }
}
