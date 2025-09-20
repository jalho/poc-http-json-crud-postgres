pub async fn get_many(
    state: axum::extract::State<crate::web::State>,
) -> Result<axum::Json<Vec<crate::db::schema::Book>>, axum::http::StatusCode> {
    let state: crate::web::State = state.0;

    let (db_tx, db_rx) = tokio::sync::oneshot::channel();
    let db_query: crate::db::Query = crate::db::Query::SelectManyBooks { respond_to: db_tx };

    if let Err(err) = state.db_client.send(db_query).await {
        log::error!("{err}");
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    };

    let db_actor_response = match db_rx.await {
        Ok(n) => n,
        Err(err) => {
            log::error!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let db_response: Vec<crate::db::schema::Book> = match db_actor_response {
        Ok(n) => n,
        Err(err) => {
            log::error!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(axum::Json(db_response))
}

pub async fn get_one_by_id(
    state: axum::extract::State<crate::web::State>,
    book_id: axum::extract::Path<uuid::Uuid>,
) -> Result<axum::Json<crate::db::schema::Book>, axum::http::StatusCode> {
    let state: crate::web::State = state.0;
    let book_id: uuid::Uuid = book_id.0;

    let (db_tx, db_rx) = tokio::sync::oneshot::channel();
    let db_query: crate::db::Query = crate::db::Query::SelectOneBookById {
        respond_to: db_tx,
        book_id,
    };

    if let Err(err) = state.db_client.send(db_query).await {
        log::error!("{err}");
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    };

    let db_actor_response = match db_rx.await {
        Ok(n) => n,
        Err(err) => {
            log::error!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let db_response: crate::db::schema::Book = match db_actor_response {
        Ok(n) => n,
        Err(err) => {
            log::error!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(axum::Json(db_response))
}

pub async fn post_one(
    state: axum::extract::State<crate::web::State>,
    book: axum::Json<crate::db::schema::Book>,
) -> Result<(), axum::http::StatusCode> {
    let state: crate::web::State = state.0;
    let book: crate::db::schema::Book = book.0;

    let (db_tx, db_rx) = tokio::sync::oneshot::channel();
    let db_query: crate::db::Query = crate::db::Query::InsertOne {
        respond_to: db_tx,
        book,
    };

    if let Err(err) = state.db_client.send(db_query).await {
        log::error!("{err}");
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    };

    let db_actor_response = match db_rx.await {
        Ok(n) => n,
        Err(err) => {
            log::error!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let _db_response: usize = match db_actor_response {
        Ok(n) => n,
        Err(err) => {
            log::error!("{err}");
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(())
}
