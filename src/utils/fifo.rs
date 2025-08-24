#[cfg(test)]
extern crate alloc;

use alloc::boxed::Box;
use core::mem::MaybeUninit;
use core::time::Duration;
use core::{
    option::{
        Option,
        Option::{None, Some},
    },
    result::{
        Result,
        Result::{Err, Ok},
    },
};

use crate::time::TimeImpl;

#[derive(Debug)]
pub enum Error {
    Full,
    Empty,
}

// Single Producer Single Consumer FIFO
//
// len      = head - tail (mod N)
// capacity = N - len (==> mod N)
//
// Initial State
//     v head
//   +---+---+---+---+
//   |   |   |   |   |
//   +---+---+---+---+
//     ^ tail
//
// Push x
//         v head
//   +---+---+---+---+
//   | x |   |   |   |
//   +---+---+---+---+
//     ^ tail
//
// Pop
//         v head
//   +---+---+---+---+
//   |  |   |   |   |
//   +---+---+---+---+
//         ^ tail
pub struct Spsc<T, const N: usize = 8> {
    head: usize,
    tail: usize,
    inner: [MaybeUninit<T>; N],
}

impl<T, const N: usize> Spsc<T, N> {
    pub fn new() -> (SpscTx<T, N>, SpscRx<T, N>) {
        let fifo = Box::into_raw(Box::new(Spsc {
            head: 0,
            tail: 0,
            inner: unsafe { MaybeUninit::uninit().assume_init() },
        }));

        (SpscTx::new(fifo), SpscRx::new(fifo))
    }

    pub fn reset(&mut self) {
        self.head = 0;
        self.tail = 0;
    }

    pub fn len(&self) -> usize {
        (N + self.head - self.tail) % N
    }

    pub fn capacity(&self) -> usize {
        N - self.len()
    }

    pub(crate) fn enqueue(&mut self, data: T) -> Result<(), Error> {
        if self.capacity() == 0 {
            return Err(Error::Full);
        }

        let head = self.head;
        self.inner[head].write(data);

        self.head = (head + 1) % N;

        Ok(())
    }

    pub(crate) fn dequeue(&mut self) -> Option<&T> {
        if self.len() == 0 {
            return None;
        }

        let tail = self.tail;
        let data = unsafe { self.inner[tail].assume_init_ref() };

        self.tail = (tail + 1) % N;

        Some(data)
    }
}

pub struct SpscTx<T, const N: usize> {
    fifo: *mut Spsc<T, N>,
}

impl<T, const N: usize> SpscTx<T, N> {
    fn new(fifo: *mut Spsc<T, N>) -> SpscTx<T, N> {
        SpscTx { fifo }
    }

    /// Enqueue an item into the FIFO
    /// # Examples
    /// ```
    /// use srobo_base::utils::fifo::Spsc;
    /// let (tx, rx) = Spsc::<u8, 4>::new();
    /// tx.enqueue(1);
    /// assert_eq!(rx.dequeue(), Some(&1));
    /// ```
    pub fn enqueue(&self, data: T) -> Result<(), Error> {
        unsafe { (*self.fifo).enqueue(data) }
    }

    pub fn reset(&self) {
        unsafe {
            (*self.fifo).reset();
        }
    }
}

pub struct SpscRx<T, const N: usize> {
    fifo: *mut Spsc<T, N>,
}

impl<T, const N: usize> SpscRx<T, N> {
    fn new(fifo: *mut Spsc<T, N>) -> SpscRx<T, N> {
        SpscRx { fifo }
    }

    /// Dequeue an item from the FIFO
    /// Returns None if the FIFO is empty
    /// # Examples
    /// ```
    /// use srobo_base::utils::fifo::Spsc;
    /// let (tx, rx) = Spsc::<u8, 4>::new();
    /// assert_eq!(rx.dequeue(), None);
    /// ```
    ///
    /// ```
    /// use srobo_base::utils::fifo::Spsc;
    /// let (tx, rx) = Spsc::<u8, 4>::new();
    /// tx.enqueue(1);
    /// assert_eq!(rx.dequeue(), Some(&1));
    /// ```
    pub fn dequeue(&self) -> Option<&T> {
        return unsafe { (*self.fifo).dequeue() };
    }

    /// Dequeue an item from the FIFO
    /// Returns None if the FIFO is empty
    /// # Examples
    /// ```
    /// use srobo_base::utils::fifo::Spsc;
    /// let (tx, rx) = Spsc::<u8, 4>::new();
    /// assert_eq!(rx.len(), 0);
    /// ```
    ///
    /// ```
    /// use srobo_base::utils::fifo::Spsc;
    /// let (tx, rx) = Spsc::<u8, 4>::new();
    /// tx.enqueue(1);
    /// assert_eq!(rx.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        unsafe { (*self.fifo).len() }
    }

    pub fn wait_available(&self, timeout: Duration, time: &impl TimeImpl) -> bool {
        let start = time.now();

        loop {
            if self.len() > 0 {
                return true;
            }

            if time.now() - start > timeout {
                return false;
            }

            time.sleep(Duration::from_millis(10));
        }
    }
}

impl<T: Copy, const N: usize> SpscRx<T, N> {
    pub fn touch(&self) -> Option<T> {
        self.dequeue().copied()
    }
}
