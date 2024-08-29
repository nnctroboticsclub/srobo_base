use core::option::Option;
use core::option::Option::None;
use core::option::Option::Some;

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

pub struct Lined {
    buf: [u8; 1024],
    len: usize,
}

impl Lined {
    pub fn new() -> Lined {
        Lined {
            buf: [0; 1024],
            len: 0,
        }
    }

    pub fn feed(&mut self, data: &[u8]) {
        let length = self.len;

        self.buf[length..length + data.len()].copy_from_slice(data);
        self.len += data.len();
    }

    pub fn get_line(&mut self) -> Option<Vec<u8>> {
        let pos = self.buf.iter().position(|&x| x == b'\n').map(|x| x + 1);

        if let None = pos {
            return None;
        }
        let pos = pos.unwrap();

        if pos > self.len {
            return None;
        }

        let mut buf = vec![0; pos];
        buf.copy_from_slice(&self.buf[..pos]);

        self.buf.copy_within(pos..self.len, 0);
        self.len -= pos;

        Some(buf)
    }
}
