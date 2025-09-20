//! Function naming convention: HTTP method (conveying create-read-update-delete
//! semantics), followed by scalar (e.g. `one` or `many`), followed by selector
//! (e.g. `by_id`).

pub async fn post_one(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
    axum::extract::Path(genre): axum::extract::Path<api::Genre>,
    axum::Json(book): axum::Json<api::BookUnpopulated>,
) -> axum::http::StatusCode {
    let id: uuid::Uuid = uuid::Uuid::new_v4();
    let book = book.populate(id, genre);

    let _rows_affected: usize = match shared.db_client.insert_book(book).await {
        Ok(n) => n,
        Err(_) => {
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    axum::http::StatusCode::NO_CONTENT
}

pub async fn get_all(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
) -> Result<axum::Json<Vec<api::BookPopulated>>, axum::http::StatusCode> {
    let all_books: Vec<crate::db::schema_v1::Book> = match shared.db_client.select_books_not_removed().await {
        Ok(n) => n,
        Err(_) => {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let all_books: Vec<api::BookPopulated> = all_books.into_iter().map(|n| n.into()).collect();

    Ok(axum::Json(all_books))
}

pub async fn get_one_by_id(
    axum::extract::State(mut shared): axum::extract::State<crate::web::Shared>,
    axum::extract::Path(book_id): axum::extract::Path<uuid::Uuid>,
) -> Result<axum::Json<api::BookPopulated>, axum::http::StatusCode> {
    let book: crate::db::schema_v1::Book = match shared.db_client.select_book_by_id(book_id).await {
        Ok(n) => n,
        Err(_) => {
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if let Some(removed_at_utc) = book.removed_at_utc {
        log::error!("Forbidden: Cannot GET: Book {book_id} was removed at {removed_at_utc} UTC");
        return Err(axum::http::StatusCode::FORBIDDEN);
    }

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
        log::error!("Bad request: Cannot DELETE: Book {book_id} already removed at {removed_at_utc} UTC");
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BookPopulated {
        /// Metadata.
        pub id: uuid::Uuid,

        pub title: String,
        pub genre: String,
        /// None if the page count stored in the database does not fit in an
        /// unsigned 16-bit integer.
        pub page_count: Option<u16>,
    }

    /// HTTP API schema. Not to be confused with the database schema. Separation is
    /// useful to allow the two to evolve independently of each other.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BookUnpopulated {
        pub title: String,
        pub page_count: u16,
    }

    impl BookUnpopulated {
        pub fn populate(self, id: uuid::Uuid, genre: Genre) -> crate::db::schema_v1::Book {
            crate::db::schema_v1::Book {
                id,
                removed_at_utc: None,

                title: self.title,
                genre: genre.to_string(),
                page_count: self.page_count.into(),
            }
        }
    }

    impl From<crate::db::schema_v1::Book> for BookPopulated {
        fn from(db: crate::db::schema_v1::Book) -> Self {
            Self {
                id: db.id,
                title: db.title,
                genre: {
                    let genre: Genre = db.genre.into();
                    genre.to_string()
                },
                page_count: db.page_count.try_into().ok(),
            }
        }
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(non_camel_case_types)]
    pub enum Genre {
        history,
        horror,
        scifi,
        scitech,
        other,
    }

    impl std::fmt::Display for Genre {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Genre::history => write!(f, "History"),
                Genre::horror => write!(f, "Horror"),
                Genre::scifi => write!(f, "Science Fiction"),
                Genre::scitech => write!(f, "Science and Techonology"),
                Genre::other => write!(f, "Other"),
            }
        }
    }

    impl From<String> for Genre {
        fn from(value: String) -> Self {
            match value.as_str() {
                "History" => Self::history,
                "Horror" => Self::horror,
                "Science Fiction" => Self::scifi,
                "Science and Techonology" => Self::scitech,
                _ => Self::other,
            }
        }
    }
}
