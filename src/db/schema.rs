diesel::table! {
    books (id) {
        // UUID PRIMARY KEY
        id -> Uuid,

        // TIMESTAMP WITHOUT TIME ZONE NULL
        removed_at_utc -> Nullable<Timestamp>,

        // VARCHAR(256) NOT NULL
        title -> Varchar,
    }
}

#[derive(
    serde::Deserialize,
    serde::Serialize,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Selectable,
    diesel::Insertable,
    Debug,
    PartialEq,
    Clone,
)]
#[diesel(table_name = books)]
pub struct Book {
    /*
     * Meta data.
     */
    pub id: uuid::Uuid,
    pub removed_at_utc: Option<chrono::NaiveDateTime>,

    /*
     * Data.
     */
    pub title: String,
    /*
     * TODO: Add column page_count: u16
     */
    /*
     * TODO: Add:
     *       - publish_date: chrono::NaiveDate, i.e. TIMESTAMP WITHOUT TIME ZONE
     *       - publish_date_timezone: chrono_tz::Tz, i.e. VARCHAR(128)
     *
     *       Also, make the publish_date* data into one structure somehow. Maybe
     *       define a custom composite PostgreSQL type named "book_publish_date"
     *       consisting of the date and timezone i.e. TIMESTAMP WITHOUT TIME
     *       ZONE and VARCHAR(128) respectively, or if that's not easily
     *       expressable in `diesel` then maybe use JSONB type or some unnamed
     *       tuple of TIMESTAMP WITHOUT TIME ZONE and VARCHAR(128) if that's a
     *       thing in PostgreSQL...
     */
}
