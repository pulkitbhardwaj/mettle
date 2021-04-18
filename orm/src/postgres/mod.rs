mod connection;
mod driver;

pub use connection::*;
pub use driver::*;

use tokio_postgres::Error;

use crate::database::Driver;

pub async fn connect(config: &str) -> Result<impl Driver, Error> {
    PGDriver::new(config).await
}
