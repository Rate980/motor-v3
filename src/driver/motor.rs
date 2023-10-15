pub trait Motor {
    type Error;
    const MAX_SPEED: f64;
    const MIN_SPEED: f64;
    fn forward(&mut self) -> Result<(), Self::Error>;
    fn backward(&mut self) -> Result<(), Self::Error>;
    fn stop(&mut self) -> Result<(), Self::Error>;
    fn short_brake(&mut self) -> Result<(), Self::Error>;
    fn set_speed(&mut self, speed: f64) -> Result<(), Self::Error>;
}
