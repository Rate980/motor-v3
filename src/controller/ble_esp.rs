// use std::{
//     sync::{mpsc, OnceLock},
//     thread,
// };

// use serialport::SerialPort;

// use crate::driver::Communication;

// use super::{BleHook, Distance};

// struct BleEsp<P: SerialPort, B: BleHook + Clone + Send> {
//     port: P,
//     hook: B,
//     thread: OnceLock<thread::JoinHandle<()>>,
//     channel: (mpsc::Sender<f64>, mpsc::Receiver<f64>),
// }

// impl<P: SerialPort, B: BleHook + Clone + Send> BleEsp<P, B> {
//     pub fn new(port: P, hook: B) -> Self {
//         Self {
//             port,
//             hook,
//             thread: OnceLock::new(),
//             channel: mpsc::channel(),
//         }
//     }

//     pub fn start_thread(&mut self) {
//         let tx = self.channel.0.clone();
//         let mut port = self.port.try_clone().unwrap();
//         let mut hook = self.hook.clone();

//         self.thread.set(thread::spawn(move || loop {
//             let line = port.read_line();
//             if let Ok(line) = line {
//                 match &line[..1] {
//                     "0" => {
//                         hook.on_disconnect();
//                     }
//                     "1" => {
//                         hook.on_connect();
//                     }

//                     _ => (),
//                 }
//             }
//         }));
//     }
// }

// impl<P: SerialPort + Communication, B: BleHook + Clone + Send> Distance for BleEsp<P, B> {
//     type Error = P::Error;
//     fn get_distance(&mut self) -> Result<f64, Self::Error> {
//         todo!();
//     }
// }
