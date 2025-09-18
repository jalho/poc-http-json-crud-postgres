mod handlers;

pub struct Actor {
    listen_address: String,
    router: axum::Router,
}

impl Actor {
    pub fn init(listen_address: &str) -> Self {
        let router: axum::Router = axum::Router::new().route("/", axum::routing::get(handlers::get_foos::handle_request));
        Self {
            router,
            listen_address: listen_address.to_owned(),
        }
    }

    pub async fn work(self) -> Summary {
        todo!();
    }
}

pub struct Summary;
