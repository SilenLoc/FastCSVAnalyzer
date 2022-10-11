
use std::fs::File;
use std::io::Result;
use std::io::{BufReader, Lines};



pub fn analyze(
    predicate: impl Fn(String, Vec<String>) -> (bool, String),
    handle_action: impl Fn(String) -> (),
    in_action: impl Fn() -> Result<Lines<BufReader<File>>>,
    values_to_analyze_against: Vec<String>
) {
    if let Ok(lines) = in_action() {
        for line in lines.into_iter() {
            if let Ok(line) = line {
                let (is_predicate, tested_line) = predicate(line, values_to_analyze_against.clone());
                if is_predicate {
                    handle_action(tested_line.clone())
                }
            }
        }
    }
}