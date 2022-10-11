extern crate core;



use log::info;

use crate::init::init;
use crate::logging::init_logger;
use crate::reader::read_lines;
use crate::analyze::analyze;

mod init;
mod logging;
mod reader;
mod analyze;

fn main() {
    init();

    let predicate = |str, values| my_check_action2(str, values);
    let handle_action = |string| my_out(string);
    let in_action = || read_lines(String::from("src/resources/x.csv"));

    let values_to_analyze_against = vec!["someValue".to_string(), "someOther".to_string()];

    analyze(predicate, handle_action, in_action, values_to_analyze_against)
}



fn my_check_action2(string: String, values_to_analyze_against: Vec<String>) -> (bool, String) {
    
    let split = string.split(";");
    let as_string_vec: Vec<&str> = split.into_iter().collect();

    let result_vec: Vec<bool> = values_to_analyze_against
        .into_iter()
        .map(|value| as_string_vec.contains(&value.as_str()))
        .collect();

    let result = result_vec
        .into_iter()
        .all(|result_per_string| result_per_string);

    (result, string)
}


fn my_out(string: String) -> () {
    info!("{}", string);
}
