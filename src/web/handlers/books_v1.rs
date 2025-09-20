//! Function naming convention: HTTP method (conveying create-read-update-delete
//! semantics), followed by scalar (e.g. `one` or `many`), followed by selector
//! (e.g. `by_id`).

pub async fn post_one(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
    axum::Json(book): axum::Json<api::Book>,
) -> axum::http::StatusCode {
    let _rows_affected: usize = match shared.db_client.insert_book((&book).into()).await {
        Ok(n) => n,
        Err(_) => {
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    return axum::http::StatusCode::NO_CONTENT;
}

pub async fn get_all(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
) -> Result<axum::Json<Vec<api::Book>>, axum::http::StatusCode> {
    let all_books: Vec<crate::db::schema_v1::Book> = match shared.db_client.select_books_all().await {
        Ok(n) => n,
        Err(_) => {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let all_books: Vec<api::Book> = all_books.iter().map(|n| n.into()).collect();

    Ok(axum::Json(all_books))
}

pub async fn get_one_by_id(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
    axum::extract::Path(book_id): axum::extract::Path<uuid::Uuid>,
) -> Result<axum::Json<api::Book>, axum::http::StatusCode> {
    let book: crate::db::schema_v1::Book = match shared.db_client.select_book_by_id(book_id).await {
        Ok(n) => n,
        Err(_) => {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(axum::Json((&book).into()))
}

pub async fn delete_one_by_id(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
    axum::extract::Path(book_id): axum::extract::Path<uuid::Uuid>,
) -> axum::http::StatusCode {
    let existing: crate::db::schema_v1::Book = match shared.db_client.select_book_by_id(book_id).await {
        Ok(n) => n,
        Err(_) => {
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    if let Some(removed_at_utc) = existing.removed_at_utc {
        log::error!("Bad request: Cannot remove book {book_id}: Already removed at {removed_at_utc} UTC");
        return axum::http::StatusCode::BAD_REQUEST;
    }

    let removal_instant: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
    let _rows_affected: usize = match shared.db_client.update_book_set_removed(book_id, removal_instant).await {
        Ok(n) => n,
        Err(_) => {
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    return axum::http::StatusCode::NO_CONTENT;
}

mod api {
    /// HTTP API schema. Not to be confused with the database schema. Separation is
    /// useful to allow the two to evolve independently of each other.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Book {
        pub id: uuid::Uuid,
        pub removed_at_utc: Option<chrono::NaiveDateTime>,
        pub title: String,
    }

    impl From<&crate::db::schema_v1::Book> for Book {
        fn from(value: &crate::db::schema_v1::Book) -> Self {
            Self {
                id: value.id,
                removed_at_utc: value.removed_at_utc,
                title: value.title.clone(),
            }
        }
    }

    impl Into<crate::db::schema_v1::Book> for &Book {
        fn into(self) -> crate::db::schema_v1::Book {
            crate::db::schema_v1::Book {
                id: self.id,
                removed_at_utc: self.removed_at_utc,
                title: self.title.clone(),
            }
        }
    }
}
