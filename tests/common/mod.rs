use std::{fs::File, io::Result};
use tempfile::tempfile;

pub fn setup() -> Result<File> {
    tempfile()
}
