

pub mod Matchers {

    pub trait Matcher {
        fn reduce(&self, result_vec: Vec<bool>) -> bool;
    }
    pub struct All;

    impl Matcher for All {
        fn reduce(&self, result_vec: Vec<bool>) -> bool {
            result_vec
            .into_iter()
            .all(|result_per_string| result_per_string)
        }
    }
}