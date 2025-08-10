mod read_stream;
mod write_stream;

use core::time::Duration;
use std::sync::{
    mpsc::{self, Receiver},
    Arc, Mutex,
};

use read_stream::SDevReadableStream;
use write_stream::SDevWritableStream;

use crate::communication::AsyncSerial;

pub(super) enum SerialDeviceRequest {
    Tx(Vec<u8>),
    OnRx(Box<dyn Fn(&[u8]) + Send>),
    OnClose(Box<dyn Fn() + Send>),
    Close(),
}

pub struct SerialDevice {
    pub(super) req_tx: mpsc::Sender<SerialDeviceRequest>,
}
pub type SerialDeviceRef = Arc<Mutex<SerialDevice>>;

impl SerialDevice {
    pub fn new(port: String, baud_rate: u32) -> SerialDeviceRef {
        let (req_tx, req_rx) = mpsc::channel();

        let obj = Arc::new(Mutex::new(SerialDevice { req_tx }));

        std::thread::spawn(move || {
            serial_device_thread(port, baud_rate, req_rx);
        });

        obj
    }
}

// TODO(syoch): Call OnClose() immediately when any errors happend
fn serial_device_thread(port: String, baud_rate: u32, req_rx: Receiver<SerialDeviceRequest>) -> () {
    let mut rx_callbacks = Vec::new();
    let mut close_callbacks = Vec::new();

    let mut serial = serialport::new(&port, baud_rate)
        .baud_rate(baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open serial port");
    let mut buf = [0; 1024];

    loop {
        match req_rx.recv_timeout(Duration::from_millis(10)) {
            Ok(SerialDeviceRequest::Tx(data)) => {
                serial.write(&data).unwrap();
                serial.flush().unwrap();
            }
            Ok(SerialDeviceRequest::OnRx(cb)) => {
                rx_callbacks.push(cb);
            }
            Ok(SerialDeviceRequest::OnClose(cb)) => {
                close_callbacks.push(cb);
            }
            Ok(SerialDeviceRequest::Close()) => {
                break;
            }
            Err(_) => (),
        };

        match serial.read(&mut buf) {
            Ok(n) => {
                if n > 0 {
                    let data = &buf[..n];
                    for cb in rx_callbacks.iter_mut() {
                        cb(data);
                    }
                }
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::TimedOut => continue,
                _ => {
                    println!("Error reading from serial port: {}", e);
                    break;
                }
            },
        }
    }

    for cb in close_callbacks {
        cb();
    }
}

impl Drop for SerialDevice {
    fn drop(&mut self) {
        let _ = self.req_tx.send(SerialDeviceRequest::Close());
    }
}

type Error = ();
impl AsyncSerial<Error> for SerialDeviceRef {
    type ReadStream = SDevReadableStream<Error>;
    type WriteStream = SDevWritableStream<Error>;

    fn open(&self) -> Result<(Self::ReadStream, Self::WriteStream), Error> {
        Ok((
            SDevReadableStream::new(self.clone()),
            SDevWritableStream::new(self.clone()),
        ))
    }
}
