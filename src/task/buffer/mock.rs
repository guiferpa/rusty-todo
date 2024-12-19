use std::collections::HashMap;
use std::io::{self, Read, Seek, Write};
use std::usize;

use super::SetLen;

pub enum MockCaller {
    SetLen,
    Read,
    Write,
    Seek,
}

#[derive(Clone)]
pub struct MockBuffer {
    call_counter: HashMap<&'static str, u8>,
}

impl MockBuffer {
    pub fn new() -> MockBuffer {
        MockBuffer {
            call_counter: HashMap::new(),
        }
    }

    pub fn call_times(&self, caller: MockCaller) -> u8 {
        match caller {
            MockCaller::SetLen => *self.call_counter.get("set_len").unwrap_or(&0),
            MockCaller::Read => *self.call_counter.get("read").unwrap_or(&0),
            MockCaller::Write => *self.call_counter.get("write").unwrap_or(&0),
            MockCaller::Seek => *self.call_counter.get("seek").unwrap_or(&0),
        }
    }

    fn increment_call_counter(&mut self, key: &'static str) {
        let acc: u8 = *self.call_counter.get(key).unwrap_or(&0);
        self.call_counter.insert(key, 1 + acc);
    }
}

impl SetLen for MockBuffer {
    fn set_len(&mut self, _size: u64) -> std::io::Result<()> {
        self.increment_call_counter("set_len");
        Ok(())
    }
}

impl Read for MockBuffer {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        self.increment_call_counter("read");
        Ok(0)
    }
}

impl Write for MockBuffer {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        self.increment_call_counter("write");
        Ok(0)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn by_ref(&mut self) -> &mut MockBuffer {
        self
    }

    fn write_all(&mut self, mut _buf: &[u8]) -> io::Result<()> {
        Ok(())
    }

    fn write_fmt(&mut self, _fmt: std::fmt::Arguments<'_>) -> io::Result<()> {
        Ok(())
    }

    fn write_vectored(&mut self, _bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        Ok(0)
    }
}

impl Seek for MockBuffer {
    fn seek(&mut self, _pos: io::SeekFrom) -> io::Result<u64> {
        self.increment_call_counter("seek");
        Ok(0)
    }

    fn rewind(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn seek_relative(&mut self, _offset: i64) -> io::Result<()> {
        Ok(())
    }

    fn stream_position(&mut self) -> io::Result<u64> {
        Ok(0)
    }
}
