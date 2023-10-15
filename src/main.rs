use motor_v3::{
    controller::{Machine, Seebo},
    driver::{Angle, Distance, TB6569},
    pin::MotorPin,
    stub,
};
use rppal::gpio::Gpio;

const MOTOR1_PIN: MotorPin = MotorPin {
    in1: 2,
    in2: 3,
    pwm: 18,
};
const MOTOR2_PIN: MotorPin = MotorPin {
    in1: 4,
    in2: 17,
    pwm: 13,
};

const STOP_DISTANCE: f64 = 0.0;

// fn loop_<M: Motor, A: Angle, D: Distance>(motor1: M, motor2: M, angle: A, distance: D) {

// }

fn main() {
    let gpio = Gpio::new().unwrap();
    let in1 = gpio.get(MOTOR1_PIN.in1).unwrap().into_output();
    let in2 = gpio.get(MOTOR1_PIN.in2).unwrap().into_output();
    let mut pwm = gpio.get(MOTOR1_PIN.pwm).unwrap().into_output();
    pwm.set_pwm_frequency(1000.0, 0.0).unwrap();

    let motor1 = TB6569::new(in1, in2, pwm);

    let in1 = gpio.get(MOTOR2_PIN.in1).unwrap().into_output();
    let in2 = gpio.get(MOTOR2_PIN.in2).unwrap().into_output();
    let mut pwm = gpio.get(MOTOR2_PIN.pwm).unwrap().into_output();
    pwm.set_pwm_frequency(1000.0, 0.0).unwrap();

    let motor2 = TB6569::new(in1, in2, pwm);
    let mut seebo = Seebo::new(motor1, motor2, 0.0);
    let mut angle = stub::Angle::new();
    let mut distance = stub::Distance::new();

    loop {
        let a = angle.get_angle();
        let d = distance.get_distance();

        if d < STOP_DISTANCE {
            seebo.short_brake().unwrap();
            continue;
        }

        match a {
            0b0_0001000 => seebo.forward().unwrap(),
            0b0_1000000 => {
                seebo.turn(-1.0, 1.0).unwrap();
                seebo.set_speed(0.5).unwrap()
            }
            _ => (),
        }
    }
}
