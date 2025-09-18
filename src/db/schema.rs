diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
    }
}

#[derive(serde::Serialize, diesel::Queryable, diesel::Identifiable, diesel::Selectable, Debug, PartialEq, Clone)]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
}
