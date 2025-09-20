use crate::web::handlers::books_v1;

mod handlers;

#[derive(Clone)]
struct DatabaseClient {
    tx_query: tokio::sync::mpsc::Sender<crate::db::Query>,
}

impl DatabaseClient {
    pub fn new(tx_query: tokio::sync::mpsc::Sender<crate::db::Query>) -> Self {
        Self { tx_query }
    }

    pub async fn select_books_all(&mut self) -> Result<Vec<crate::db::schema::Book>, ()> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let db_query: crate::db::Query = crate::db::Query::SelectManyBooks { respond_to: tx };

        if let Err(err) = self.tx_query.send(db_query).await {
            log::error!("{err}");
            return Err(());
        };

        let db_actor_response = match rx.await {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        let books: Vec<crate::db::schema::Book> = match db_actor_response {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        return Ok(books);
    }

    pub async fn select_book_by_id(&mut self, book_id: uuid::Uuid) -> Result<crate::db::schema::Book, ()> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let db_query: crate::db::Query = crate::db::Query::SelectOneBookById {
            respond_to: tx,
            book_id: book_id,
        };

        if let Err(err) = self.tx_query.send(db_query).await {
            log::error!("{err}");
            return Err(());
        };

        let db_actor_response = match rx.await {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        let book: crate::db::schema::Book = match db_actor_response {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        return Ok(book);
    }

    pub async fn insert_book(&mut self, book: crate::db::schema::Book) -> Result<usize, ()> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let db_query: crate::db::Query = crate::db::Query::InsertOne { respond_to: tx, book };

        if let Err(err) = self.tx_query.send(db_query).await {
            log::error!("{err}");
            return Err(());
        };

        let db_actor_response = match rx.await {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        let rows_affected: usize = match db_actor_response {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        return Ok(rows_affected);
    }
}

#[derive(Clone)]
struct Shared {
    db_client: DatabaseClient,
}

impl Shared {
    pub fn init(tx_query: tokio::sync::mpsc::Sender<crate::db::Query>) -> Self {
        Self {
            db_client: DatabaseClient::new(tx_query),
        }
    }
}

pub struct Actor {
    term: crate::term::Handle,

    listen_address: String,
    router: axum::Router,
}

impl Actor {
    pub fn init(
        term: crate::term::Handle,
        listen_address: &str,
        tx_query: tokio::sync::mpsc::Sender<crate::db::Query>,
    ) -> Self {
        let state: Shared = Shared::init(tx_query);

        let router: axum::Router = axum::Router::new()
            .route("/api/books/v1", axum::routing::post(books_v1::post_one))
            .route("/api/books/v1", axum::routing::get(books_v1::get_many))
            .route("/api/books/v1/{id}", axum::routing::get(books_v1::get_one_by_id))
            .with_state(state);

        Self {
            term,

            router,
            listen_address: listen_address.to_owned(),
        }
    }

    pub async fn work(self) -> Summary {
        let listener: tokio::net::TcpListener = match tokio::net::TcpListener::bind(self.listen_address).await {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                self.term
                    .trigger_termination(crate::term::TriggerGlobalCancellation::WebServer)
                    .await;
                return Summary;
            }
        };

        self.term
            .token()
            .run_until_cancelled(async {
                if let Err(err) = axum::serve(listener, self.router).await {
                    /*
                     * From axum's docs (v0.8.4):
                     *
                     * > Although this future resolves to `io::Result<()>`, it
                     * > will never actually complete or return an error.
                     */
                    unreachable!("axum::serve never completes or returns an error: {err}");
                };
            })
            .await;

        Summary
    }
}

pub struct Summary;
