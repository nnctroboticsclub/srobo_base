#[cfg(test)]
extern crate alloc;

use core::cell::RefCell;

use super::fifo::{self, Spsc, SpscRx, SpscTx};

pub struct StringQueue<const N: usize, const L: usize> {}

impl<const N: usize, const L: usize> StringQueue<N, L> {
    pub fn new() -> (StringQueueTx<N, L>, StringQueueRx<N, L>) {
        let (tx, rx) = Spsc::<u8, N>::new();
        let (tx_len, rx_len) = Spsc::<usize, L>::new();

        let tx = StringQueueTx {
            ch: tx,
            len: tx_len,
        };
        let rx = StringQueueRx {
            ch: rx,
            len: rx_len,
            buffer: RefCell::new([0; N]),
        };
        (tx, rx)
    }
}

pub struct StringQueueTx<const N: usize, const L: usize> {
    ch: SpscTx<u8, N>,
    len: SpscTx<usize, L>,
}

impl<const N: usize, const L: usize> StringQueueTx<N, L> {
    pub fn enqueue(&self, data: &[u8]) -> Result<(), fifo::Error> {
        self.len.enqueue(data.len())?;
        for c in data {
            self.ch.enqueue(c.clone())?;
        }
        Ok(())
    }
}

pub struct StringQueueRx<const N: usize, const L: usize> {
    ch: SpscRx<u8, N>,
    len: SpscRx<usize, L>,
    buffer: RefCell<[u8; N]>,
}

impl<const N: usize, const L: usize> StringQueueRx<N, L> {
    fn load(&self) -> Option<usize> {
        let len = match self.len.dequeue() {
            Some(len) => len,
            None => return None,
        }
        .clone();

        for i in 0..len {
            self.buffer.borrow_mut()[i] = match self.ch.dequeue() {
                Some(c) => c.clone(),
                None => return None,
            };
        }

        Some(len)
    }

    pub fn dequeue(&self) -> Option<&[u8]> {
        let len = match self.load() {
            Some(len) => len,
            None => return None,
        };

        let ret = unsafe {
            let ptr: *mut u8 = self.buffer.as_ptr() as *mut u8;
            core::slice::from_raw_parts(ptr, len)
        };

        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_queue() {
        let (tx, rx) = StringQueue::<16, 16>::new();

        tx.enqueue("hello".as_bytes()).unwrap();
        tx.enqueue("world".as_bytes()).unwrap();
        tx.enqueue("aiueo".as_bytes()).unwrap();
        assert_eq!(rx.dequeue(), Some("hello".as_bytes()));
        assert_eq!(rx.dequeue(), Some("world".as_bytes()));
        assert_eq!(rx.dequeue(), Some("aiueo".as_bytes()));
        assert_eq!(rx.dequeue(), None);
    }
}
