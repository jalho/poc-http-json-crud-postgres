pub async fn get_all(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
) -> Result<axum::Json<Vec<crate::db::schema::Book>>, axum::http::StatusCode> {
    let all_books: Vec<crate::db::schema::Book> = match shared.db_client.select_books_all().await {
        Ok(n) => n,
        Err(_) => {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(axum::Json(all_books))
}

pub async fn get_one_by_id(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
    axum::extract::Path(book_id): axum::extract::Path<uuid::Uuid>,
) -> Result<axum::Json<crate::db::schema::Book>, axum::http::StatusCode> {
    let book: crate::db::schema::Book = match shared.db_client.select_book_by_id(book_id).await {
        Ok(n) => n,
        Err(_) => {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(axum::Json(book))
}

pub async fn post_one(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
    axum::Json(book): axum::Json<crate::db::schema::Book>,
) -> axum::http::StatusCode {
    let _rows_affected: usize = match shared.db_client.insert_book(book).await {
        Ok(n) => n,
        Err(_) => {
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    return axum::http::StatusCode::NO_CONTENT;
}
