#[cfg(test)]
pub mod mock;

use std::{fs::File, io};

pub trait SetLen {
    fn set_len(&mut self, size: u64) -> io::Result<()>;
}

impl SetLen for File {
    fn set_len(&mut self, size: u64) -> io::Result<()> {
        File::set_len(&self, size)
    }
}

