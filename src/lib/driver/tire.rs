pub trait TireI {
    type Error;
    fn forward(&self) -> Result<(), Self::Error>;
    fn backward(&self) -> Result<(), Self::Error>;
    async fn set_speed(&self, speed: u8) -> Result<(), Self::Error>;
}

struct TB6569<PIN: OutputPin, PWM: P> {
    in1: t,
    in2: t,
    pwm: t,
}
