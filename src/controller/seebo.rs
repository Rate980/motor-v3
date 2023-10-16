use thiserror::Error;

use crate::driver::Motor;

use super::Machine;

#[derive(Debug, Error)]
pub enum Error<M: Motor> {
    #[error("motor error")]
    Motor(M::Error),
    #[error("invalid value")]
    InvalidValue,
}

pub struct Seebo<M: Motor> {
    motor1: M,
    motor2: M,
    speed: f64,
    speed_diff: (f64, f64),
}

impl<M: Motor> Seebo<M> {
    pub fn new(mut motor1: M, mut motor2: M, speed: f64) -> Self {
        motor1.set_speed(speed);
        motor2.set_speed(speed);
        Self {
            motor1,
            motor2,
            speed,
            speed_diff: (1.0, 1.0),
        }
    }
}

impl<M: Motor> Machine for Seebo<M> {
    type Error = Error<M>;

    // fn backward(&mut self) -> Result<(), Self::Error> {
    //     self.motor1.backward().map_err(Self::Error::Motor)?;
    //     self.motor2.backward().map_err(Self::Error::Motor)?;
    //     self.set_speed(self.speed)?;
    //     Ok(())
    // }

    // fn forward(&mut self) -> Result<(), Self::Error> {
    //     self.motor1.forward().map_err(Self::Error::Motor)?;
    //     self.motor2.forward().map_err(Self::Error::Motor)?;
    //     self.set_speed(self.speed)?;
    //     Ok(())
    // }

    fn stop(&mut self) -> Result<(), Self::Error> {
        self.motor1.stop().map_err(Self::Error::Motor)?;
        self.motor2.stop().map_err(Self::Error::Motor)?;
        Ok(())
    }

    fn short_brake(&mut self) -> Result<(), Self::Error> {
        self.motor1.short_brake().map_err(Self::Error::Motor)?;
        self.motor2.short_brake().map_err(Self::Error::Motor)?;
        Ok(())
    }

    fn set_speed(&mut self, speed: f64) -> Result<(), Self::Error> {
        self.speed = speed;

        let (right, left) = self.speed_diff;
        let motor1_speed = right.abs() * self.speed;
        let motor2_speed = left.abs() * self.speed;

        self.motor1
            .set_speed(motor1_speed)
            .map_err(Self::Error::Motor)?;
        self.motor2
            .set_speed(motor2_speed)
            .map_err(Self::Error::Motor)?;
        Ok(())
    }

    fn turn(&mut self, mut right: f64, left: f64) -> Result<(), Self::Error> {
        right *= -1.0;
        self.speed_diff = (right, left);
        if right.abs() > M::MAX_SPEED || left.abs() > M::MAX_SPEED {
            return Err(Self::Error::InvalidValue);
        }

        if right < 0.0 {
            self.motor1.backward().map_err(Self::Error::Motor)?;
        } else {
            self.motor1.forward().map_err(Self::Error::Motor)?;
        }

        if left < 0.0 {
            self.motor2.backward().map_err(Self::Error::Motor)?;
        } else {
            self.motor2.forward().map_err(Self::Error::Motor)?;
        }

        self.set_speed(self.speed)
    }
}
