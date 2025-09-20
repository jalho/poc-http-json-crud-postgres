//! PostgreSQL schema.

diesel::table! {
    books (id) {
        // UUID PRIMARY KEY
        id -> Uuid,

        // TIMESTAMP WITHOUT TIME ZONE NULL
        removed_at_utc -> Nullable<Timestamp>,

        // VARCHAR(256) NOT NULL
        title -> Varchar,

        // VARCHAR(256) NOT NULL
        genre -> Varchar,

        // INTEGER NOT NULL
        page_count -> Integer,
    }
}

/// Database schema. Not to be confused with the schema exposed via the HTTP
/// CRUD API. Separation is useful to allow the two to evolve independently of
/// each other.
#[derive(diesel::Queryable, diesel::Identifiable, diesel::Selectable, diesel::Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = books)]
pub struct Book {
    /// Metadata: `UUID PRIMARY KEY`.
    pub id: uuid::Uuid,
    /// Metadata: `TIMESTAMP WITHOUT TIME ZONE NULL`.
    pub removed_at_utc: Option<chrono::NaiveDateTime>,

    /// `VARCHAR(256) NOT NULL`
    pub title: String,
    /// `VARCHAR(256) NOT NULL`
    pub genre: String,
    /// `INTEGER NOT NULL` (4-byte signed integer, i.e. akin to `i32`)
    pub page_count: i32,
}
