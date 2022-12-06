use std::io::{self};

mod round_5;

fn main() -> io::Result<()> {
    round_5::start();
    Ok(())
}