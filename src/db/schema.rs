diesel::table! {
    books (id) {
        id -> Uuid,
        title -> Varchar,
    }
}

#[derive(serde::Serialize, diesel::Queryable, diesel::Identifiable, diesel::Selectable, Debug, PartialEq, Clone)]
#[diesel(table_name = books)]
pub struct Book {
    pub id: uuid::Uuid,
    pub title: String,
}
