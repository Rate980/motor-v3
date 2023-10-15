use embedded_hal::{digital::v2::OutputPin, PwmPin};

use super::motor::Motor;

#[derive(Debug)]
pub struct TB6569<T: OutputPin, U: PwmPin> {
    in1: T,
    in2: T,
    pwm: U,
}

impl<T: OutputPin, U: PwmPin> TB6569<T, U> {
    pub fn new(in1: T, in2: T, pwm: U) -> Self {
        Self { in1, in2, pwm }
    }
}

impl<T: OutputPin, U: PwmPin<Duty = f64>> Motor for TB6569<T, U> {
    const MAX_SPEED: f64 = 1.0;
    const MIN_SPEED: f64 = 0.0;
    type Error = T::Error;

    fn forward(&mut self) -> Result<(), Self::Error> {
        self.in1.set_high()?;
        self.in2.set_low()?;
        Ok(())
    }

    fn backward(&mut self) -> Result<(), Self::Error> {
        self.in1.set_low()?;
        self.in2.set_high()?;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Self::Error> {
        self.in1.set_low()?;
        self.in2.set_low()?;
        Ok(())
    }

    fn short_brake(&mut self) -> Result<(), Self::Error> {
        self.in1.set_high()?;
        self.in2.set_high()?;
        Ok(())
    }

    fn set_speed(&mut self, speed: f64) -> Result<(), Self::Error> {
        self.pwm.set_duty(speed);
        Ok(())
    }
}
