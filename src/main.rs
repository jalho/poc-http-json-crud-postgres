mod db;
mod term;
mod web;

/// Proof-of-concept implementation demonstrating:
///
/// - HTTP JSON API with ergonomics of _serde_ ecosystem and _axum_
/// - CRUD operations on PostgreSQL with ergonomics of _diesel_
/// - Using _actor pattern_ in _tokio_ ecosystem
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
fn main() -> std::process::ExitCode {
    let terminator: term::Actor = term::Actor::hook();

    let db_client: db::Actor =
        match db::Actor::connect("postgres://postgres:postgres@127.0.0.1:5432/postgres?connect_timeout=1") {
            Ok(n) => n,
            Err(err) => {
                eprintln!("{err}");
                return std::process::ExitCode::from(42);
            }
        };

    let web_server: web::Actor = web::Actor::init("127.0.0.1:8080");

    let runtime: tokio::runtime::Runtime = match tokio::runtime::Builder::new_current_thread().build() {
        Ok(n) => n,
        Err(err) => {
            eprintln!("{err}");
            return std::process::ExitCode::from(43);
        }
    };

    let _done: (db::Summary, web::Summary, term::Summary) =
        runtime.block_on(async { tokio::join!(db_client.work(), web_server.work(), terminator.work()) });

    std::process::ExitCode::SUCCESS
}
