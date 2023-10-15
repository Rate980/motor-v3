use std::fmt::Display;

use super::Distance;

struct BleEsp<P: Communication, B: BleHook> {
    port: P,
    hook: B,
}

impl<P: Communication, B: BleHook> BleEsp<P, B> {
    fn new(port: P, hook: B) -> Self {
        Self { port, hook }
    }
}

impl Distance for BleEsp {
    type Error = P::Error;
    fn get_distance(&mut self) -> Result<f64, Self::Error> {
        let line = self.port.read_line()?;
    }
}
