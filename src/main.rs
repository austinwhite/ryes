use std::env;
use std::io::{self, BufWriter, Write};

const BUFSIZE: usize = 8192;

fn main() {
    let expletive = env::args().nth(1).unwrap_or("y".into());
    let mut writer = BufWriter::with_capacity(BUFSIZE, io::stdout());
    loop {
        writeln!(writer, "{}", expletive).unwrap();
    }
}
