use std::fs::File;
use std::io::{self, Read};

pub fn read_file(filename: &str) -> io::Result<String> {
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
