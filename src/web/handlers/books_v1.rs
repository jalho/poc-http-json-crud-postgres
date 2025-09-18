pub async fn get_many(
    state: axum::extract::State<std::sync::Arc<crate::web::State>>,
) -> Result<axum::Json<Vec<crate::db::schema::Book>>, axum::http::StatusCode> {
    let state: std::sync::Arc<crate::web::State> = state.0;

    let (db_tx, db_rx) = tokio::sync::oneshot::channel();
    let db_query: crate::db::Query = crate::db::Query::SelectManyBooks { respond_to: db_tx };

    {
        let lock = state.db_client_shared.lock().await;
        if let Err(err) = lock.send(db_query).await {
            eprintln!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        };
    }

    let db_response = match db_rx.await {
        Ok(n) => n,
        Err(err) => {
            eprintln!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let query_response: Vec<crate::db::schema::Book> = match db_response {
        Ok(n) => n,
        Err(err) => {
            eprintln!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(axum::Json(query_response))
}

pub async fn get_one_by_id(
    state: axum::extract::State<std::sync::Arc<crate::web::State>>,
    book_id: axum::extract::Path<uuid::Uuid>,
) -> Result<axum::Json<crate::db::schema::Book>, axum::http::StatusCode> {
    let state: std::sync::Arc<crate::web::State> = state.0;
    let book_id: uuid::Uuid = book_id.0;

    let (db_tx, db_rx) = tokio::sync::oneshot::channel();
    let db_query: crate::db::Query = crate::db::Query::SelectOneBookById {
        respond_to: db_tx,
        book_id,
    };

    {
        let lock = state.db_client_shared.lock().await;
        if let Err(err) = lock.send(db_query).await {
            eprintln!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        };
    }

    let db_response = match db_rx.await {
        Ok(n) => n,
        Err(err) => {
            eprintln!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let query_response: crate::db::schema::Book = match db_response {
        Ok(n) => n,
        Err(err) => {
            eprintln!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(axum::Json(query_response))
}
