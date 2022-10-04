use crate::init_logger;
use log::info;

pub fn init() {
    init_logger().expect("could not init logger");
    info!("logger is ready");
}
