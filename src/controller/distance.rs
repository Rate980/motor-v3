pub trait Distance {
    type Error;
    fn get_distance(&mut self) -> Result<f64, Self::Error>;
}
