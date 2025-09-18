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
        let job = async {
            'recv: loop {
                let received: Query = match self.chan_query.1.recv().await {
                    Some(n) => n,
                    None => {
                        break 'recv;
                    }
                };
                todo!(
                    "handle query: do something with DB connection in self, and then write response back to the response sender contained in the Query"
                );
            }
        };

        self.term.token().run_until_cancelled(job).await;

        Summary
    }
}

pub struct Summary;

pub struct Query {
    respond_to: tokio::sync::oneshot::Sender<Response>,
}

impl Query {
    pub fn new(respond_to: tokio::sync::oneshot::Sender<Response>) -> Self {
        Self { respond_to }
    }
}

pub struct Response;
