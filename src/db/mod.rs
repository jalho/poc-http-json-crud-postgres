pub mod schema_v1;

use crate::db::schema_v1::books::dsl::books;

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
        log::info!("Connected to database");

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
        loop {
            let query_received: Query = match query_recv.recv().await {
                Some(n) => n,
                None => {
                    return;
                }
            };

            match query_received {
                Query::InsertBook { respond_to, book } => {
                    let query = diesel::insert_into(schema_v1::books::table).values(&book);

                    let query_dbg: String = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
                    log::debug!("{query_dbg}");

                    use diesel::RunQueryDsl;
                    let db_query_result: Result<usize, diesel::result::Error> = query.execute(db_connection);

                    if let Err(_err) = respond_to.send(db_query_result) {
                        log::error!("Failed to respond from DB client");
                    }
                }

                Query::SelectBooksAll { respond_to } => {
                    use diesel::SelectableHelper;
                    let selection = schema_v1::Book::as_select();

                    use diesel::query_dsl::methods::SelectDsl;
                    let query = books.select(selection);

                    let query_dbg: String = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
                    log::debug!("{query_dbg}");

                    use diesel::RunQueryDsl;
                    let db_query_result: Result<Vec<schema_v1::Book>, diesel::result::Error> =
                        query.load(db_connection);

                    if let Err(_err) = respond_to.send(db_query_result) {
                        log::error!("Failed to respond from DB client");
                    }
                }

                Query::SelectBookById { respond_to, book_id } => {
                    use diesel::SelectableHelper;
                    let selection = schema_v1::Book::as_select();

                    use diesel::ExpressionMethods;
                    use diesel::query_dsl::methods::FilterDsl;
                    use diesel::query_dsl::methods::SelectDsl;
                    let query = books.filter(schema_v1::books::id.eq(book_id)).select(selection);

                    let query_dbg: String = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
                    log::debug!("{query_dbg}");

                    use diesel::RunQueryDsl;
                    let db_query_result: Result<schema_v1::Book, diesel::result::Error> =
                        query.get_result(db_connection);

                    if let Err(_err) = respond_to.send(db_query_result) {
                        log::error!("Failed to respond from DB client");
                    }
                }

                Query::UpdateBookSetRemovedById {
                    respond_to,
                    book_id,
                    removed_at_utc,
                } => {
                    let without_timezone: chrono::NaiveDateTime = removed_at_utc.naive_utc();

                    use diesel::ExpressionMethods;
                    let query = diesel::update(books)
                        .filter(schema_v1::books::id.eq(book_id))
                        .set(schema_v1::books::removed_at_utc.eq(without_timezone));

                    let query_dbg: String = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
                    log::debug!("{query_dbg}");

                    use diesel::RunQueryDsl;
                    let db_query_result: Result<usize, diesel::result::Error> = query.execute(db_connection);

                    if let Err(_err) = respond_to.send(db_query_result) {
                        log::error!("Failed to respond from DB client");
                    }
                }
            }
        }
    }
}

pub struct Summary;

pub enum Query {
    InsertBook {
        respond_to: tokio::sync::oneshot::Sender<Result<usize, diesel::result::Error>>,
        book: schema_v1::Book,
    },
    SelectBooksAll {
        respond_to: tokio::sync::oneshot::Sender<Result<Vec<schema_v1::Book>, diesel::result::Error>>,
    },
    SelectBookById {
        respond_to: tokio::sync::oneshot::Sender<Result<schema_v1::Book, diesel::result::Error>>,
        book_id: uuid::Uuid,
    },
    UpdateBookSetRemovedById {
        respond_to: tokio::sync::oneshot::Sender<Result<usize, diesel::result::Error>>,
        book_id: uuid::Uuid,
        removed_at_utc: chrono::DateTime<chrono::Utc>,
    },
}
