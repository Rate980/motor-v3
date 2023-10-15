use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use motor_v3::{
    controller::{Machine, Seebo},
    driver::TB6569,
    pin::MotorPin,
};
use rppal::gpio::Gpio;
use serialport::SerialPort;

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

#[cfg(not(feature = "stub"))]
const BLE_ESP_PATH: &str = "/dev/ttyUSB0";
#[cfg(feature = "stub")]
const BLE_ESP_PATH: &str = "/dev/pts/1";
const BLE_ESP_BAUD_RATE: u32 = 9600;

#[cfg(not(feature = "stub"))]
const INFRARED_ESP_PATH: &str = "/dev/ttyUSB1";
#[cfg(feature = "stub")]
const BLE_ESP_PATH: &str = "/dev/pts/3";
const INFRARED_ESP_BAUD_RATE: u32 = 115200;

const STOP_DISTANCE: f64 = 0.0;

// fn loop_<M: Motor, A: Angle, D: Distance>(motor1: M, motor2: M, angle: A, distance: D) {

// }
//
//
#[derive(Debug, thiserror::Error)]
enum ReadLineError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

fn read_line<P: SerialPort>(port: &mut P) -> Result<String, ReadLineError> {
    let mut inp = vec![];
    'i: while {
        let mut buf = [0; 1];
        let a = match port.read(&mut buf) {
            Ok(x) => x,
            Err(err) => match err.kind() {
                std::io::ErrorKind::TimedOut => continue 'i,
                _ => return Err(err.into()),
            },
        };

        if a == 0 {
            continue 'i;
        }

        inp.push(buf[0]);
        buf[0] != b'\n'
    } {}

    Ok(String::from_utf8(inp)?.trim().to_string())
}

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

    let mut ble_serial = serialport::new(BLE_ESP_PATH, BLE_ESP_BAUD_RATE)
        // .timeout(std::time::Duration::from_millis(10))
        .open_native()
        .unwrap();

    let mut infrared_serial = serialport::new(INFRARED_ESP_PATH, INFRARED_ESP_BAUD_RATE)
        // .timeout(std::time::Duration::from_millis(10))
        .open_native()
        .unwrap();

    let angle_value = Arc::new(Mutex::new(0));
    {
        let angle_value = angle_value.clone();
        thread::spawn(move || loop {
            let inp = match read_line(&mut infrared_serial) {
                Ok(x) => x,
                Err(_) => continue,
            };
            {
                let mut angle = angle_value.lock().unwrap();

                *angle = match inp.parse::<u8>() {
                    Ok(x) => x << 1 ^ (x >> 6 & 0b1) & 0b0111_1111,
                    Err(_) => continue,
                }
            }
        });
    }

    let distances = Arc::new([Mutex::new(0.0), Mutex::new(0.0), Mutex::new(0.0)]);
    let (ble_tx, ble_rx) = mpsc::channel::<u8>();
    {
        let distances = distances.clone();
        thread::spawn(move || loop {
            let inp = match read_line(&mut ble_serial) {
                Ok(x) => x,
                Err(_) => continue,
            };

            if inp.len() <= 1 {
                ble_tx
                    .send(match inp.parse() {
                        Ok(x) => x,
                        Err(_) => continue,
                    })
                    .unwrap();
                continue;
            }

            {
                let tmp = inp
                    .split_whitespace()
                    .skip(1)
                    .map(|x| x.parse::<f64>())
                    .collect::<Result<Vec<_>, _>>();

                if let Ok(tmp) = tmp {
                    tmp.into_iter().enumerate().for_each(|(i, x)| {
                        let mut distance = distances[i].lock().unwrap();
                        *distance = x;
                    });
                }
            }
        });
    }

    loop {
        let angle = *angle_value.lock().unwrap();
        let distances = distances
            .iter()
            .map(|x| *x.lock().unwrap())
            .collect::<Vec<_>>();
        if let Ok(message) = ble_rx.try_recv() {
            println!("message: {}", message)
        }
        println!("angle: {:b}\n distance: {:?}", angle, distances);
        println!("----------------------");
    }

    // 'i: loop {
    //     {
    //         for distance in distances.iter() {
    //             let distance = distance.lock().unwrap();
    //             if *distance <= STOP_DISTANCE {
    //                 seebo.short_brake().unwrap();
    //                 continue 'i;
    //             }
    //         }
    //     }

    //     let angle = *angle_value.lock().unwrap();

    //     match angle {
    //         0b0000_1000 => seebo.forward().unwrap(),
    //         0b0100_0000 | 0b0110_0000 => {
    //             seebo.turn(-1.0, 1.0).unwrap();
    //             seebo.set_speed(0.5).unwrap()
    //         }
    //         0b0001_1000 => {
    //             seebo.turn(1.0, -1.0).unwrap();
    //             seebo.set_speed(0.5).unwrap()
    //         }
    //         0b0000_0001 => {
    //             seebo.turn(1.0, -1.0).unwrap();
    //             seebo.set_speed(0.5).unwrap()
    //         }

    //         _ => (),
    //     }
    // }
}
