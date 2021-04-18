use crate::common::Driver;

pub struct PGDriver {}

impl Driver for PGDriver {}

pub fn connect(dsn: &str) -> impl Driver {
    PGDriver {}
}
