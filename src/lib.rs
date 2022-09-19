#![allow(dead_code)]
mod reading;
pub use reading::Reading;
mod utils;
use std::fmt::Arguments;
use std::io::{stdin, stdout, BufWriter, Stdout, Write};
pub use utils::*;

pub mod prelude {
    pub use super::{Reading, RW};
    pub use std::cmp::{max, min};
}

pub struct RW(BufWriter<Stdout>);

impl RW {
    pub fn new() -> Self {
        RW(BufWriter::new(stdout()))
    }

    pub fn write_fmt<'a>(&mut self, args: Arguments<'a>) -> Result<(), ()> {
        self.0.write_fmt(args).unwrap();
        Ok(())
    }
}

impl Drop for RW {
    fn drop(&mut self) {
        self.0.flush().unwrap();
    }
}

impl Reading for RW {
    fn read_line(&mut self) -> String {
        let mut txt = String::new();
        stdin().read_line(&mut txt).unwrap();
        txt.trim().to_owned()
    }
}
