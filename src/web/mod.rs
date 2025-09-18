mod handlers;

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
        let router: axum::Router =
            axum::Router::new().route("/", axum::routing::get(handlers::get_foos::handle_request));

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
                eprintln!("{err}");
                self.term.trigger_termination().await;
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
