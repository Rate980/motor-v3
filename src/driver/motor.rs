pub trait Motor {
    type Error;
    type Speed;
    fn forward(&mut self) -> Result<(), Self::Error>;
    fn backward(&mut self) -> Result<(), Self::Error>;
    fn stop(&mut self) -> Result<(), Self::Error>;
    fn short_brake(&mut self) -> Result<(), Self::Error>;
    fn set_speed(&mut self, speed: Self::Speed) -> Result<(), Self::Error>;
}
