use std::thread::sleep;
use std::time::Duration;

use motor_v3::{
    controller::{Machine, Seebo},
    driver::{Motor, TB6569},
};
use rppal::gpio::Gpio;

fn main() {
    let gpio = Gpio::new().unwrap();
    let in1 = gpio.get(2).unwrap().into_output();
    let in2 = gpio.get(3).unwrap().into_output();
    // let mut pwm = Pwm::with_frequency(Channel::Pwm0, 100.0, 0.0, Polarity::Normal, true).unwrap();
    let mut pwm = gpio.get(12).unwrap().into_output();
    pwm.set_pwm_frequency(100.0, 1.0).unwrap();
    let motor1 = TB6569::new(in1, in2, pwm);
    let in1 = gpio.get(4).unwrap().into_output();
    let in2 = gpio.get(17).unwrap().into_output();
    let mut pwm = gpio.get(13).unwrap().into_output();
    pwm.set_pwm_frequency(100.0, 1.0).unwrap();
    let motor2 = TB6569::new(in1, in2, pwm);

    let mut seebo = Seebo::new(motor1, motor2, 1.0);
    // seebo.stop().unwrap();

    // println!("forward");
    // seebo.forward().unwrap();
    // sleep(Duration::from_secs(3));

    // println!("backward");
    // seebo.backward().unwrap();
    // sleep(Duration::from_secs(3));

    // println!("stop");
    // seebo.stop().unwrap();

    // println!("forward");
    // seebo.forward().unwrap();
    // sleep(Duration::from_secs(3));

    // println!("short brake");
    // seebo.short_brake().unwrap();

    seebo.stop().unwrap();

    loop {
        sleep(Duration::from_millis(100))
    }
}
