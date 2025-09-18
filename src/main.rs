mod db;
mod web;

fn main() -> std::process::ExitCode {
    let db: db::Actor =
        match db::Actor::connect("postgres://postgres:postgres@127.0.0.1:5432/postgres?connect_timeout=1") {
            Ok(n) => n,
            Err(err) => {
                eprintln!("{err}");
                return std::process::ExitCode::from(42);
            }
        };

    let runtime: tokio::runtime::Runtime = match tokio::runtime::Builder::new_current_thread().build() {
        Ok(n) => n,
        Err(err) => {
            eprintln!("{err}");
            return std::process::ExitCode::from(43);
        }
    };

    std::process::ExitCode::SUCCESS
}
