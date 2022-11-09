use crate::{outs::my_out, predicates::ContainsValues, matcher::Matchers::All, reader::read_lines, analyze::analyze, };


pub fn test_csv_analyze(){
    let handle_action = |string| my_out(string);
    let in_action = || read_lines(String::from("src/resources/x.csv"));

    let predicate_and_value = ContainsValues {
        values: vec!["someValue".to_string(), "someOther".to_string()],
        delimeter: ';'
    };

    let all = All;

    analyze(predicate_and_value,all, handle_action, in_action);
}