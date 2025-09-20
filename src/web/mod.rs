use crate::web::handlers::books_v1;

mod db_client;
mod handlers;

#[derive(Clone)]
struct Shared {
    db_client: db_client::DatabaseClient,
}

impl Shared {
    pub fn init(tx_query: tokio::sync::mpsc::Sender<crate::db::Query>) -> Self {
        Self {
            db_client: db_client::DatabaseClient::new(tx_query),
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
            /*
             * Create-read-update-delete (CRUD) API for books, v1.
             */
            .route("/api/books/v1", axum::routing::post(books_v1::post_one))
            .route("/api/books/v1", axum::routing::get(books_v1::get_all))
            .route("/api/books/v1/{id}", axum::routing::get(books_v1::get_one_by_id))
            .route("/api/books/v1/{id}", axum::routing::delete(books_v1::delete_one_by_id))
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
