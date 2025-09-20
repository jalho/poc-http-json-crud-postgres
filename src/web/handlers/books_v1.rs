pub async fn get_many(
    state: axum::extract::State<crate::web::Shared>,
) -> Result<axum::Json<Vec<crate::db::schema::Book>>, axum::http::StatusCode> {
    let mut state: crate::web::Shared = state.0;

    let all_books: Vec<crate::db::schema::Book> = match state.db_client.select_books_all().await {
        Ok(n) => n,
        Err(_) => {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(axum::Json(all_books))
}

pub async fn get_one_by_id(
    state: axum::extract::State<crate::web::Shared>,
    book_id: axum::extract::Path<uuid::Uuid>,
) -> Result<axum::Json<crate::db::schema::Book>, axum::http::StatusCode> {
    let mut state: crate::web::Shared = state.0;
    let book_id: uuid::Uuid = book_id.0;

    let book: crate::db::schema::Book = match state.db_client.select_book_by_id(book_id).await {
        Ok(n) => n,
        Err(_) => {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(axum::Json(book))
}

pub async fn post_one(
    state: axum::extract::State<crate::web::Shared>,
    book: axum::Json<crate::db::schema::Book>,
) -> Result<(), axum::http::StatusCode> {
    let mut state: crate::web::Shared = state.0;
    let book: crate::db::schema::Book = book.0;

    let _rows_affected: usize = match state.db_client.insert_book(book).await {
        Ok(n) => n,
        Err(_) => {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(())
}
