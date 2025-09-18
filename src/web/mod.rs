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
        /*
         * TODO: Activate global termination signal in case of errors!
         */
        let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(self.listen_address).await.unwrap();

        /*
         * TODO: Run until global termination signal canceled!
         */
        axum::serve(listener, self.router).await.unwrap();
        todo!();
    }
}

pub struct Summary;
