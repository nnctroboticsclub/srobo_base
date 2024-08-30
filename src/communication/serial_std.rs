use core::time::Duration;
use std::sync::mpsc;

use super::{AsyncReadableStream, AsyncSerial, WritableStream};

enum SerialDeviceMessage {
    Tx(Vec<u8>),
    OnRx(Box<dyn Fn(&[u8])>),
}

unsafe impl Send for SerialDeviceMessage {}

type Error = ();

pub struct SDevWritableStream {
    chan_tx: mpsc::Sender<SerialDeviceMessage>,
}

impl SDevWritableStream {
    fn new(chan_tx: mpsc::Sender<SerialDeviceMessage>) -> SDevWritableStream {
        SDevWritableStream { chan_tx }
    }
}

impl WritableStream for SDevWritableStream {
    type Error = Error;

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.chan_tx
            .send(SerialDeviceMessage::Tx(data.to_vec()))
            .unwrap();
        Ok(())
    }
}

pub struct SDevReadableStream {
    chan_tx: mpsc::Sender<SerialDeviceMessage>,
}

impl SDevReadableStream {
    fn new(chan_tx: mpsc::Sender<SerialDeviceMessage>) -> SDevReadableStream {
        SDevReadableStream { chan_tx }
    }
}

impl AsyncReadableStream for SDevReadableStream {
    type Error = Error;

    fn on_data(&mut self, cb: Box<dyn Fn(&[u8])>) -> Result<(), Self::Error> {
        self.chan_tx.send(SerialDeviceMessage::OnRx(cb)).unwrap();
        Ok(())
    }
}

pub struct SerialDevice {
    chan_tx: mpsc::Sender<SerialDeviceMessage>,
}

impl SerialDevice {
    pub fn new(port: String, baud_rate: u32) -> SerialDevice {
        let (tx, rx) = mpsc::channel();

        let obj = SerialDevice { chan_tx: tx };

        std::thread::spawn(move || {
            let mut buf = [0; 1024];
            let mut dev = serialport::new(&port, baud_rate)
                .baud_rate(baud_rate)
                .timeout(Duration::from_millis(10))
                .open()
                .expect("Failed to open serial port");

            let mut callbacks = vec![];

            loop {
                match rx.recv_timeout(Duration::from_millis(10)) {
                    Ok(SerialDeviceMessage::Tx(data)) => {
                        dev.write(&data).unwrap();
                        dev.flush().unwrap();
                    }
                    Ok(SerialDeviceMessage::OnRx(cb)) => {
                        callbacks.push(cb);
                    }
                    Err(_) => (),
                };

                match dev.read(&mut buf) {
                    Ok(n) => {
                        for cb in &mut callbacks {
                            cb(&buf[..n]);
                        }
                    }
                    _ => (),
                }
            }
        });

        obj
    }
}

impl AsyncSerial<Error> for SerialDevice {
    type ReadStream = SDevReadableStream;
    type WriteStream = SDevWritableStream;

    fn open(&self) -> Result<(Self::ReadStream, Self::WriteStream), Error> {
        Ok((
            SDevReadableStream::new(self.chan_tx.clone()),
            SDevWritableStream::new(self.chan_tx.clone()),
        ))
    }
}
