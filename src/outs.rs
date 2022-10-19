use log::info;


pub(crate) fn my_out(string: String) -> () {
    info!("{}", string);
}