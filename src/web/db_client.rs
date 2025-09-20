#[derive(Clone)]
pub struct DatabaseClient {
    tx_query: tokio::sync::mpsc::Sender<crate::db::Query>,
}

impl DatabaseClient {
    pub fn new(tx_query: tokio::sync::mpsc::Sender<crate::db::Query>) -> Self {
        Self { tx_query }
    }

    pub async fn select_books_all(&mut self) -> Result<Vec<crate::db::schema_v1::Book>, ()> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let db_query: crate::db::Query = crate::db::Query::SelectBooksAll { respond_to: tx };

        if let Err(err) = self.tx_query.send(db_query).await {
            log::error!("{err}");
            return Err(());
        };

        let db_actor_response = match rx.await {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        let books: Vec<crate::db::schema_v1::Book> = match db_actor_response {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        Ok(books)
    }

    pub async fn select_book_by_id(&mut self, book_id: uuid::Uuid) -> Result<crate::db::schema_v1::Book, ()> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let db_query: crate::db::Query = crate::db::Query::SelectBookById {
            respond_to: tx,
            book_id,
        };

        if let Err(err) = self.tx_query.send(db_query).await {
            log::error!("{err}");
            return Err(());
        };

        let db_actor_response = match rx.await {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        let book: crate::db::schema_v1::Book = match db_actor_response {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        Ok(book)
    }

    pub async fn insert_book(&mut self, book: crate::db::schema_v1::Book) -> Result<usize, ()> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let db_query: crate::db::Query = crate::db::Query::InsertBook { respond_to: tx, book };

        if let Err(err) = self.tx_query.send(db_query).await {
            log::error!("{err}");
            return Err(());
        };

        let db_actor_response = match rx.await {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        let rows_affected: usize = match db_actor_response {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        Ok(rows_affected)
    }

    pub async fn update_book_set_removed(
        &mut self,
        book_id: uuid::Uuid,
        removed_at_utc: chrono::DateTime<chrono::Utc>,
    ) -> Result<usize, ()> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let db_query: crate::db::Query = crate::db::Query::UpdateBookSetRemovedById {
            respond_to: tx,
            book_id,
            removed_at_utc,
        };

        if let Err(err) = self.tx_query.send(db_query).await {
            log::error!("{err}");
            return Err(());
        };

        let db_actor_response = match rx.await {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        let rows_affected: usize = match db_actor_response {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        Ok(rows_affected)
    }
}
