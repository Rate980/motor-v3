pub trait Machine {
    type Error;

    fn forward(&mut self) -> Result<(), Self::Error> {
        self.turn(1.0, 1.0)
    }
    fn backward(&mut self) -> Result<(), Self::Error> {
        self.turn(-1.0, -1.0)
    }
    fn stop(&mut self) -> Result<(), Self::Error>;
    fn short_brake(&mut self) -> Result<(), Self::Error>;
    fn turn(&mut self, right: f64, left: f64) -> Result<(), Self::Error>;
    fn set_speed(&mut self, speed: f64) -> Result<(), Self::Error>;
}
