use diesel::{Connection, SelectableHelper};

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
    }
}

#[derive(diesel::Queryable, diesel::Identifiable, diesel::Selectable, Debug, PartialEq, Clone)]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
}

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

                /*
                 * TODO: Using the already established PostgreSQL connection, do SELECT all `Book`s.
                 */
                use crate::db::books::dsl::books;
                use diesel::RunQueryDsl;
                use diesel::query_dsl::methods::SelectDsl;
                let result: Result<Vec<Book>, diesel::result::Error> =
                    books.select(Book::as_select()).load(&mut self.connection);
                let response: Response = Response::new(result);
                if let Err(_err) = received.respond_to.send(response) {
                    eprintln!("failed to respond from DB client");
                }
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

pub struct Response(pub Result<Vec<Book>, diesel::result::Error>);

impl Response {
    pub fn new(result: Result<Vec<Book>, diesel::result::Error>) -> Self {
        Self(result)
    }
}
