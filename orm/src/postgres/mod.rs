mod connection;
mod driver;

pub use connection::*;
pub use driver::*;

pub fn connect(dsn: &str) -> impl Driver {
    PGDriver {}
}
