pub async fn handle_request(
    state: axum::extract::State<std::sync::Arc<crate::web::State>>,
) -> Result<&'static str, axum::http::StatusCode> {
    let state: std::sync::Arc<crate::web::State> = state.0;

    let (db_tx, db_rx) = tokio::sync::oneshot::channel();
    let db_query: crate::db::Query = crate::db::Query::new(db_tx);

    {
        let lock = state.db_client_shared.lock().await;
        if let Err(err) = lock.send(db_query).await {
            eprintln!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        };
    }

    let db_response = match db_rx.await {
        Ok(n) => n.0,
        Err(err) => {
            eprintln!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let query_response: Vec<crate::db::Book> = match db_response {
        Ok(n) => n,
        Err(err) => {
            eprintln!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok("Hello world!")
}
