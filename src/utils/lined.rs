use core::option::Option;
use core::option::Option::None;

extern crate alloc;
use super::fifo;
use super::string_queue::StringQueue;
use super::string_queue::StringQueueRx;
use super::string_queue::StringQueueTx;
use alloc::vec::Vec;

pub struct Lined<const N: usize> {
    buf: [u8; N],
    len: usize,

    queue_tx: StringQueueTx<256, 4>,
    queue_rx: StringQueueRx<256, 4>,
}

impl<const N: usize> Lined<N> {
    pub fn new() -> Lined<N> {
        let (queue_tx, queue_rx) = StringQueue::new();
        Lined {
            buf: [0; N],
            len: 0,
            queue_tx,
            queue_rx,
        }
    }

    /// Reset the buffer and the queue
    /// # Example
    /// ```
    /// use srobo_base::utils::lined::Lined;
    /// let mut lined = Lined::<16>::new();
    /// lined.feed(b"TheString");
    /// let data = lined.reset_queue();
    /// assert_eq!(data, b"TheString");
    /// ```
    pub fn reset_queue(&mut self) -> Vec<u8> {
        let mut data = Vec::new();
        if self.len > 0 {
            data.extend_from_slice(&self.buf[..self.len]);
        }
        self.queue_tx.reset();
        self.buf = [0; N];
        data
    }

    /// Feed data into the buffer
    pub fn feed(&mut self, data: &[u8]) -> Result<(), fifo::Error> {
        let length = self.len;

        if length + data.len() > self.buf.len() {
            return Err(fifo::Error::Full);
        }
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

    /// Find a line in the buffer
    /// # Example
    /// ```
    /// use srobo_base::utils::lined::Lined;
    /// let mut lined = Lined::<16>::new();
    /// lined.feed(b"Hello\nWorld\n");
    /// assert_eq!(lined.get_line().unwrap(), b"Hello\n");
    /// assert_eq!(lined.get_line().unwrap(), b"World\n");
    /// assert_eq!(lined.get_line(), None);
    /// ```
    pub fn get_line(&mut self) -> Option<&[u8]> {
        self.queue_rx.dequeue()
    }
}
