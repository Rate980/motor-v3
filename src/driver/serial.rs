use std::io::BufReader;

use serialport::SerialPort;

pub struct Serial<P: SerialPort> {
    port: BufReader<P>,
}

impl<P: SerialPort> Serial<P> {
    pub fn new(port: P) -> Self {
        Self {
            port: BufReader::new(port),
        }
    }

    pub fn with_baud_rate(mut port: P, baud_rate: u32) -> serialport::Result<Self> {
        port.set_baud_rate(baud_rate)?;
        Ok(Self::new(port))
    }
}

// impl<P: SerialPort> Communication for Serial<P> {
//     type Error = serialport::Error;
//     fn read_line(&mut self) -> Result<String, Self::Error> {
//         let mut buf = String::new();
//         self.port.read_line(&mut buf)
//     }
// }
