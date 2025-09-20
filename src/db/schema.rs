diesel::table! {
    books (id) {
        id -> Uuid,
        title -> Varchar,
        removed_at_utc -> Nullable<Timestamp>,
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
    pub id: uuid::Uuid,
    pub title: String,
    pub removed_at_utc: Option<chrono::NaiveDateTime>,
}
