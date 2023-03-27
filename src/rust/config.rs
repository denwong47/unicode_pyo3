use lazy_static::lazy_static;
use std::thread::available_parallelism;

lazy_static! {
    pub(crate) static ref CPU_COUNT: usize = available_parallelism().unwrap().get();
}
