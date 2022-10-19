


pub trait Predicate{
    fn predicate(&self,string: String) -> (bool, String);
}


pub struct PredicateAndValue {
    pub values: Vec<String>,
}

impl Predicate for PredicateAndValue {

    fn predicate(&self,
        string: String,
    ) -> (bool, String) {
        let split = string.split(";");
        let as_string_vec: Vec<&str> = split.into_iter().collect();

        let result_vec: Vec<bool> = self.values.clone()
            .into_iter()
        .map(|value| as_string_vec.contains(&value.as_str()))
            .collect();

        let result = result_vec
            .into_iter()
            .all(|result_per_string| result_per_string);

        (result, string)
    }

}
