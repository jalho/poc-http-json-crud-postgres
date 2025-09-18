mod db;
mod logg;
mod term;
mod web;

/// Proof-of-concept implementation demonstrating:
///
/// - HTTP JSON API with ergonomics of _serde_ ecosystem and _axum_
///
/// - CRUD operations on PostgreSQL with ergonomics of _diesel_
///
/// - Using _actor pattern_ in _tokio_ ecosystem (inspired by Alice Ryhl:
///   _Actors with Tokio_, RustLab Conference 2022)
///
/// ### Cheatsheet
///
/// - Starting a containerized PostgreSQL instance:
///
///   ```console
///   podman run --rm \
///     --name poc-postgres \
///     -e POSTGRES_PASSWORD=postgres \
///     -p 127.0.0.1:5432:5432/tcp \
///     docker.io/library/postgres:17.6-trixie@sha256:feff5b24fedd610975a1f5e743c51a4b360437f4dc3a11acf740dcd708f413f6
///   ```
///
/// - Creating a table named `books` in the containerized PostgreSQL instance:
///
///   ```console
///   podman exec -it poc-postgres psql -U postgres -d postgres -c '
///     CREATE TABLE books (
///       id    UUID PRIMARY KEY,
///       title VARCHAR NOT NULL
///     );'
///   ```
///
/// - POST a book:
///
///   ```console
///   curl http://127.0.0.1:8080/api/books/v1 --json '{"id":"00000000-0000-0000-0000-000000000000","title":"Foo Bar!"}'
///   ```
///
/// - GET books:
///
///   ```console
///   curl http://127.0.0.1:8080/api/books/v1
///   ```
///
///   ```json
///   [{"id":"00000000-0000-0000-0000-000000000000","title":"Foo Bar!"}]
///   ```

fn main() -> std::process::ExitCode {
    if let Err(code) = logg::initialize_logger(log::LevelFilter::Trace) {
        return code;
    }

    let terminator: term::Actor = term::Actor::hook();

    let db_client: db::Actor = match db::Actor::connect(
        terminator.get_handle(),
        "postgres://postgres:postgres@127.0.0.1:5432/postgres?connect_timeout=1",
    ) {
        Ok(n) => n,
        Err(err) => {
            log::error!("{err}");
            return std::process::ExitCode::from(44);
        }
    };

    let web_server: web::Actor = web::Actor::init(terminator.get_handle(), "127.0.0.1:8080", db_client.get_handle());

    let runtime: tokio::runtime::Runtime = match tokio::runtime::Builder::new_current_thread().enable_io().build() {
        Ok(n) => n,
        Err(err) => {
            log::error!("{err}");
            return std::process::ExitCode::from(45);
        }
    };

    let _done: (db::Summary, web::Summary, term::Summary) =
        runtime.block_on(async { tokio::join!(db_client.work(), web_server.work(), terminator.work()) });

    std::process::ExitCode::SUCCESS
}
