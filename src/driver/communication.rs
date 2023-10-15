pub trait Communication {
    type Error;
    fn next_line(&mut self) -> Result<String, Self::Error>;
    fn write_line(&mut self, line: &str) -> Result<(), Self::Error>;
}
