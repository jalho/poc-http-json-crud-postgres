use diesel::Connection;

pub struct Actor {
    connection: diesel::PgConnection,
}

impl Actor {
    pub fn connect(connection_string: &str) -> Result<Self, diesel::ConnectionError> {
        let connection: diesel::PgConnection = diesel::pg::PgConnection::establish(connection_string)?;
        Ok(Self { connection })
    }
}
