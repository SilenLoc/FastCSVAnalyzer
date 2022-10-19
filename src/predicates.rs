pub trait Predicate {
    fn check(&self, string: String) -> (bool, String);
}

pub struct ContainsValues {
    pub values: Vec<String>,
    pub delimeter: char,
}

impl Predicate for ContainsValues {

    fn check(&self, string: String,) -> (bool, String) {
        let split = string.split(self.delimeter);
        let as_string_vec: Vec<&str> = split.into_iter().collect();

        let result_vec: Vec<bool> = self
            .values
            .clone()
            .into_iter()
            .map(|value| as_string_vec.contains(&value.as_str()))
            .collect();

        let result = result_vec
            .into_iter()
            .all(|result_per_string| result_per_string);

        (result, string)
    }
}

