pub async fn handle_request(
        state: axum::extract::State<std::sync::Arc<crate::web::State>>,
) -> &'static str {
    "Hello world!"
}
