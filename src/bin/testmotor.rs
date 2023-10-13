use std::thread::sleep;
use std::time::Duration;

use embedded_hal::PwmPin;
use motor_v3::driver::{Motor, TB6569};
use rppal::gpio::Gpio;
use rppal::pwm::{Channel, Polarity, Pwm};

fn main() {
    let gpio = Gpio::new().unwrap();
    let in1 = gpio.get(2).unwrap().into_output();
    let in2 = gpio.get(3).unwrap().into_output();
    // let mut pwm = Pwm::with_frequency(Channel::Pwm0, 100.0, 0.0, Polarity::Normal, true).unwrap();
    let mut pwm = gpio.get(12).unwrap().into_output();
    pwm.set_pwm_frequency(100.0, 1.0).unwrap();
    let mut motor = TB6569::new(in1, in2, pwm);
    motor.set_speed(1.0).unwrap();
    // let mut is_up = true;
    // let mut duty = 0.0;
    //
    println!("forward");
    motor.forward().unwrap();
    sleep(Duration::from_secs(2));

    println!("stop");
    motor.stop().unwrap();
    sleep(Duration::from_secs(1));

    println!("backward");
    motor.backward().unwrap();
    sleep(Duration::from_secs(2));

    println!("forward");
    motor.forward().unwrap();
    sleep(Duration::from_secs(2));

    println!("stop");
    motor.stop().unwrap();
    sleep(Duration::from_secs(1));

    println!("forward");
    motor.forward().unwrap();
    sleep(Duration::from_secs(2));
    println!("short brake");

    motor.short_brake().unwrap();
    sleep(Duration::from_secs(1));

    loop {
        sleep(Duration::from_millis(100))
    }
}
