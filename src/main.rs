extern crate core;



use log::info;

use crate::init::init;
use crate::logging::init_logger;
use crate::reader::read_lines;

mod init;
mod logging;
mod reader;

fn main() {
    init();
    analyze(|str| my_check_action2(str), |string| my_out(string))
}







fn analyze(predicate: impl Fn(String) -> (bool, String), handle_action: impl Fn(String) -> ()) {
    if let Ok(lines) = read_lines(String::from("src/resources/x.csv")) {
        for line in lines {
            if let Ok(line) = line {
                let (is_predicate, tested_line) = predicate(line);
                if is_predicate {
                    handle_action(tested_line.clone())
                }
            }
        }
    }
}



fn my_check_action2(string: String) -> (bool, String) {

let x: Vec<String> = vec!["someValue".to_string(), "someOther".to_string(), "valueThatShouldNotBeIn".to_string()];

let split = string.split(";");
let as_string_vec: Vec<&str> = split.into_iter().collect();

    
let result_vec: Vec<bool> = x.into_iter().map(|value| as_string_vec.contains(&value.as_str())).collect();

let result =  result_vec.into_iter().all(|result_per_string|result_per_string);

(result, string)
}


fn my_check_action(string: String) -> (bool, String) {
    let split = string.split(";");
    let as_string_vec: Vec<&str> = split.into_iter().collect();
    (as_string_vec.contains(&"someValue"), string)
}

fn my_out(string: String) -> () {
    info!("{}", string);
}


