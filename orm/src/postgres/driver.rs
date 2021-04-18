use crate::database::Driver;

use tokio_postgres::{connect, tls::NoTlsStream, Client, Connection, Error, NoTls, Socket};

pub struct PGDriver {
    client: Client,
    connection: Connection<Socket, NoTlsStream>,
}

impl PGDriver {
    pub async fn new(config: &str) -> Result<impl Driver, Error> {
        let (client, connection) = connect(config, NoTls).await?;

        Ok(PGDriver { client, connection })
    }
}

impl Driver for PGDriver {}
