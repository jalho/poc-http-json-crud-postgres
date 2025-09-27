mod db;
mod logg;
mod term;
mod web;

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

    /*
     * Runtime for non-blocking workloads.
     */
    let runtime: tokio::runtime::Runtime = match tokio::runtime::Builder::new_current_thread().enable_io().build() {
        Ok(n) => n,
        Err(err) => {
            log::error!("{err}");
            return std::process::ExitCode::from(45);
        }
    };

    /*
     * Dedicated OS thread for blocking workloads.
     */
    let blocking_workloads_thread = std::thread::spawn(|| {
        let runtime: tokio::runtime::Runtime = match tokio::runtime::Builder::new_current_thread().enable_io().build() {
            Ok(n) => n,
            Err(err) => {
                log::error!("{err}");
                return Err(std::process::ExitCode::from(46));
            }
        };
        let done: db::Summary = runtime.block_on(db_client.work());
        Ok(done)
    });

    let _non_blocking_workloads_done: (web::Summary, term::Summary) =
        runtime.block_on(async { tokio::join!(web_server.work(), terminator.work()) });

    let _blocking_workloads_done: db::Summary = match blocking_workloads_thread.join() {
        Ok(Ok(n)) => n,
        Ok(Err(code)) => {
            return code;
        }
        Err(err) => {
            log::error!("{err:?}");
            return std::process::ExitCode::from(47);
        }
    };

    std::process::ExitCode::SUCCESS
}
