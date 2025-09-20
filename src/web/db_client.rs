#[derive(Clone)]
pub struct DatabaseClient {
    tx_query: tokio::sync::mpsc::Sender<crate::db::Query>,
}

impl DatabaseClient {
    pub fn new(tx_query: tokio::sync::mpsc::Sender<crate::db::Query>) -> Self {
        Self { tx_query }
    }

    pub async fn select_books_all(&mut self) -> Result<Vec<crate::db::schema::Book>, ()> {
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

        let books: Vec<crate::db::schema::Book> = match db_actor_response {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        return Ok(books);
    }

    pub async fn select_book_by_id(&mut self, book_id: uuid::Uuid) -> Result<crate::db::schema::Book, ()> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let db_query: crate::db::Query = crate::db::Query::SelectBookById {
            respond_to: tx,
            book_id: book_id,
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

        let book: crate::db::schema::Book = match db_actor_response {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(());
            }
        };

        return Ok(book);
    }

    pub async fn insert_book(&mut self, book: crate::db::schema::Book) -> Result<usize, ()> {
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

        return Ok(rows_affected);
    }
}
