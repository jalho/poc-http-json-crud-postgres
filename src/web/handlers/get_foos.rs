pub async fn handle_request(state: axum::extract::State<std::sync::Arc<crate::web::State>>) -> &'static str {
    let state: std::sync::Arc<crate::web::State> = state.0;

    {
        let lock = state.db_client_shared.lock().await;
    }

    "Hello world!"
}
