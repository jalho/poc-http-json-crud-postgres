pub mod schema;

use crate::db::schema::books::dsl::books;
use diesel::RunQueryDsl;
use diesel::query_dsl::methods::SelectDsl;
use diesel::{Connection, SelectableHelper};

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
            'recv_queries: loop {
                let query_received: Query = match self.chan_query.1.recv().await {
                    Some(n) => n,
                    None => {
                        break 'recv_queries;
                    }
                };

                match query_received {
                    Query::SelectManyBooks { respond_to } => {
                        let db_query_result: Result<Vec<schema::Book>, diesel::result::Error> =
                            books.select(schema::Book::as_select()).load(&mut self.connection);

                        if let Err(_err) = respond_to.send(Response::new(db_query_result)) {
                            eprintln!("failed to respond from DB client");
                        }
                    }
                }
            }
        };

        self.term.token().run_until_cancelled(job).await;

        Summary
    }
}

pub struct Summary;

pub enum Query {
    SelectManyBooks {
        respond_to: tokio::sync::oneshot::Sender<Response>,
    },
}

impl Query {
    pub fn select_many_books(respond_to: tokio::sync::oneshot::Sender<Response>) -> Self {
        Self::SelectManyBooks { respond_to }
    }
}

pub struct Response(pub Result<Vec<schema::Book>, diesel::result::Error>);

impl Response {
    pub fn new(result: Result<Vec<schema::Book>, diesel::result::Error>) -> Self {
        Self(result)
    }
}
