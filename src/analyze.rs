use std::fs::File;
use std::io::Result;
use std::io::{BufReader, Lines};

use crate::predicates::Predicate;

pub fn analyze<T: Predicate>(
    predicate: T,
    handle_action: impl Fn(String) -> (),
    in_action: impl Fn() -> Result<Lines<BufReader<File>>>,
) -> T {
    if let Ok(lines) = in_action() {
        for line in lines.into_iter() {
            if let Ok(line) = line {
                let (predicate_true, tested_line) = predicate.check(line);
                if predicate_true {
                    handle_action(tested_line.clone())
                }
            }
        }
    }
    predicate
}
