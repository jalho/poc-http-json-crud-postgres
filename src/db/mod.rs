pub mod schema;

use crate::db::schema::books::dsl::books;

pub struct Actor {
    term: crate::term::Handle,

    db_connection: diesel::PgConnection,

    tx_query: tokio::sync::mpsc::Sender<Query>,
    rx_query: tokio::sync::mpsc::Receiver<Query>,
}

impl Actor {
    pub fn connect(term: crate::term::Handle, connection_string: &str) -> Result<Self, diesel::ConnectionError> {
        use diesel::Connection;
        let db_connection: diesel::PgConnection = diesel::pg::PgConnection::establish(connection_string)?;

        let (tx_query, rx_query) = tokio::sync::mpsc::channel::<Query>(1);

        Ok(Self {
            term,

            db_connection,

            tx_query,
            rx_query,
        })
    }

    pub fn get_handle(&self) -> tokio::sync::mpsc::Sender<Query> {
        self.tx_query.clone()
    }

    pub async fn work(mut self) -> Summary {
        self.term
            .token()
            .run_until_cancelled(Self::handle_queries(&mut self.db_connection, &mut self.rx_query))
            .await;

        Summary
    }

    async fn handle_queries(
        db_connection: &mut diesel::PgConnection,
        query_recv: &mut tokio::sync::mpsc::Receiver<Query>,
    ) -> () {
        {
            let query_received: Query = match query_recv.recv().await {
                Some(n) => n,
                None => {
                    return ();
                }
            };

            match query_received {
                Query::SelectManyBooks { respond_to } => {
                    use diesel::SelectableHelper;
                    let selection = schema::Book::as_select();

                    use diesel::query_dsl::methods::SelectDsl;
                    let query = books.select(selection);

                    let query_dbg: String = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
                    println!("{query_dbg}");

                    use diesel::RunQueryDsl;
                    let db_query_result: Result<Vec<schema::Book>, diesel::result::Error> = query.load(db_connection);

                    if let Err(_err) = respond_to.send(Response::new(db_query_result)) {
                        eprintln!("failed to respond from DB client");
                    }
                }
            }
        }
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
