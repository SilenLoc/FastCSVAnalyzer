use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};


pub fn read_lines(filename: String) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    Ok(reader.lines())
}
