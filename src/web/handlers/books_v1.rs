//! Function naming convention: HTTP method (conveying create-read-update-delete
//! semantics), followed by scalar (e.g. `one` or `many`), followed by selector
//! (e.g. `by_id`).

pub async fn post_one(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
    axum::extract::Path(genre): axum::extract::Path<String>,
    axum::Json(book): axum::Json<api::BookUntagged>,
) -> axum::http::StatusCode {
    let id: uuid::Uuid = uuid::Uuid::new_v4();
    let book: api::BookTagged = book.populate(id, genre);

    let _rows_affected: usize = match shared.db_client.insert_book(book.into()).await {
        Ok(n) => n,
        Err(_) => {
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    axum::http::StatusCode::NO_CONTENT
}

pub async fn get_all(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
) -> Result<axum::Json<Vec<api::BookTagged>>, axum::http::StatusCode> {
    let all_books: Vec<crate::db::schema_v1::Book> = match shared.db_client.select_books_all().await {
        Ok(n) => n,
        Err(_) => {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let all_books: Vec<api::BookTagged> = all_books.into_iter().map(|n| n.into()).collect();

    Ok(axum::Json(all_books))
}

pub async fn get_one_by_id(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
    axum::extract::Path(book_id): axum::extract::Path<uuid::Uuid>,
) -> Result<axum::Json<api::BookTagged>, axum::http::StatusCode> {
    let book: crate::db::schema_v1::Book = match shared.db_client.select_book_by_id(book_id).await {
        Ok(n) => n,
        Err(_) => {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(axum::Json(book.into()))
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

    axum::http::StatusCode::NO_CONTENT
}

mod api {
    /// HTTP API schema. Not to be confused with the database schema. Separation is
    /// useful to allow the two to evolve independently of each other.
    ///
    /// Tagged means fully qualified structure.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BookTagged {
        /// Metadata.
        pub id: uuid::Uuid,
        /// Metadata.
        pub removed_at_utc: Option<chrono::NaiveDateTime>,

        pub title: String,
        pub genre: String,
        /// None if the page count stored in the database does not fit in an
        /// unsigned 16-bit integer.
        pub page_count: Option<u16>,
    }

    /// HTTP API schema. Not to be confused with the database schema. Separation is
    /// useful to allow the two to evolve independently of each other.
    ///
    /// Untagged means some pieces of information missing.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BookUntagged {
        pub title: String,
        pub page_count: u16,
    }

    impl BookUntagged {
        pub fn populate(self, id: uuid::Uuid, genre: String) -> BookTagged {
            BookTagged {
                id,
                removed_at_utc: None,
                title: self.title,
                genre,
                page_count: self.page_count.into(),
            }
        }
    }

    impl From<crate::db::schema_v1::Book> for BookTagged {
        fn from(db: crate::db::schema_v1::Book) -> Self {
            Self {
                id: db.id,
                removed_at_utc: db.removed_at_utc,
                title: db.title,
                genre: db.genre,
                page_count: db.page_count.try_into().ok(),
            }
        }
    }

    impl From<BookTagged> for crate::db::schema_v1::Book {
        fn from(api: BookTagged) -> Self {
            crate::db::schema_v1::Book {
                id: api.id,
                removed_at_utc: api.removed_at_utc,
                title: api.title,
                genre: api.genre,
                page_count: match api.page_count {
                    Some(n) => n.into(),
                    None => 0,
                },
            }
        }
    }
}
