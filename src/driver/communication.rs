pub trait Communication {
    type Error;
    fn next_line(&mut self) -> Result<String, Self::Error>;
}
