use std::io::{BufRead, BufReader};

use serialport::SerialPort;

pub trait Communication {
    type Error;
    fn read_line(&mut self) -> Result<String, Self::Error>;
    fn write_line(&mut self, line: &str) -> Result<(), Self::Error>;
}

// impl<T> Communication for T
// where
//     T: SerialPort + ?Sized,
// {
//     type Error = serialport::Error;

//     fn read_line(&mut self) -> Result<String, Self::Error> {
//         let mut port_buf = BufReader::new(self.try_clone()?);
//         let mut buf = String::new();

//         port_buf.read_line(&mut buf)?;
//         Ok(buf.trim().to_string())
//     }

//     fn write_line(&mut self, line: &str) -> Result<(), Self::Error> {
//         self.write_all(line.as_bytes())?;
//         self.write_all(b"\n")?;
//         Ok(())
//     }
// }
