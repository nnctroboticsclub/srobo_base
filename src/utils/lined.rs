use core::option::Option;
use core::option::Option::None;

extern crate alloc;

use super::fifo;
use super::string_queue::StringQueue;
use super::string_queue::StringQueueRx;
use super::string_queue::StringQueueTx;

pub struct Lined {
    buf: [u8; 64],
    len: usize,

    queue_tx: StringQueueTx<512, 4>,
    queue_rx: StringQueueRx<512, 4>,
}

impl Lined {
    pub fn new() -> Lined {
        let (queue_tx, queue_rx) = StringQueue::new();
        Lined {
            buf: [0; 64],
            len: 0,
            queue_tx,
            queue_rx,
        }
    }

    pub fn feed(&mut self, data: &[u8]) -> Result<(), fifo::Error> {
        let length = self.len;

        self.buf[length..length + data.len()].copy_from_slice(data);
        self.len += data.len();

        while self.find_line()? {}

        Ok(())
    }

    fn find_line(&mut self) -> Result<bool, fifo::Error> {
        let pos = self.buf.iter().position(|&x| x == b'\n').map(|x| x + 1);

        if let None = pos {
            return Ok(false);
        }
        let pos = pos.unwrap();

        if pos > self.len {
            return Ok(false);
        }

        self.queue_tx.enqueue(&self.buf[..pos])?;

        self.buf.copy_within(pos..self.len, 0);
        self.len -= pos;

        Ok(true)
    }

    pub fn get_line(&mut self) -> Option<&[u8]> {
        self.queue_rx.dequeue()
    }
}
