use crate::web::handlers::books_v1;

mod handlers;

struct State {
    db_client_shared: tokio::sync::Mutex<tokio::sync::mpsc::Sender<crate::db::Query>>,
}

impl State {
    pub fn init(db_client_shared: tokio::sync::mpsc::Sender<crate::db::Query>) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self {
            db_client_shared: tokio::sync::Mutex::new(db_client_shared),
        })
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
        db_client: tokio::sync::mpsc::Sender<crate::db::Query>,
    ) -> Self {
        let state: std::sync::Arc<State> = State::init(db_client);

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
