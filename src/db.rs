use diesel::Connection;

pub struct Actor {
    term: crate::term::Handle,

    connection: diesel::PgConnection,
    chan_query: (tokio::sync::mpsc::Sender<Query>, tokio::sync::mpsc::Receiver<Query>),
}

impl Actor {
    pub fn connect(term: crate::term::Handle, connection_string: &str) -> Result<Self, diesel::ConnectionError> {
        let connection: diesel::PgConnection = diesel::pg::PgConnection::establish(connection_string)?;

        Ok(Self {
            term,

            connection,
            chan_query: tokio::sync::mpsc::channel::<Query>(1),
        })
    }

    pub fn get_handle(&self) -> tokio::sync::mpsc::Sender<Query> {
        self.chan_query.0.clone()
    }

    pub async fn work(mut self) -> Summary {
        'recv: loop {
            let received: Query = match self.chan_query.1.recv().await {
                Some(n) => n,
                None => {
                    break 'recv;
                }
            };
        }

        Summary
    }
}

pub struct Summary;

pub struct Query;
