extern crate core;

use std::io::Split;

use log::info;

use crate::init::init;
use crate::logging::init_logger;
use crate::reader::read_lines;

mod init;
mod logging;
mod reader;

fn main() {
    init();
    analyze(|str| my_predicate(str))
}

fn analyze(predicate: impl Fn(&&str) -> bool) {
    if let Ok(lines) = read_lines(String::from("src/resources/x.csv")) {
        for line in lines {
            if let Ok(line) = line {
                let values = line.split(';');

                let x: Vec<&str> = values.into_iter().filter(|value| predicate(value)).collect();

                info!("{}", x.join(";"))
            }
        }
    }
}

fn my_predicate(string: &&str) -> bool {
    string.contains("someValue")
}
