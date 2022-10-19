extern crate core;

use crate::analyze::analyze;
use crate::init::init;
use crate::logging::init_logger;
use crate::outs::my_out;
use crate::predicates::PredicateAndValue;
use crate::reader::read_lines;

mod analyze;
mod init;
mod logging;
mod outs;
mod predicates;
mod reader;

fn main() {
    init();

    let handle_action = |string| my_out(string);
    let in_action = || read_lines(String::from("src/resources/x.csv"));

    let predicate_and_value: PredicateAndValue = PredicateAndValue {
        values: vec!["someValue".to_string(), "someOther".to_string()],
    };

    analyze(predicate_and_value, handle_action, in_action);
}
