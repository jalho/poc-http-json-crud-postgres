pub fn initialize_logger(level: log::LevelFilter) -> Result<log4rs::Handle, std::process::ExitCode> {
    const APPENDER_NAME_STDOUT: &str = "stdout";

    let appender_stdout = log4rs::append::console::ConsoleAppender::builder()
        .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
            "{highlight({d(%Y-%m-%d %H:%M:%S)(utc)} UTC [{level}] {message})} {file}:{line}\n",
        )))
        .build();

    let appender_cfg_stdout =
        log4rs::config::Appender::builder().build(APPENDER_NAME_STDOUT, Box::new(appender_stdout));

    let config = match log4rs::Config::builder().appender(appender_cfg_stdout).build(
        log4rs::config::Root::builder()
            .appender(APPENDER_NAME_STDOUT)
            .build(level),
    ) {
        Ok(n) => n,
        Err(err) => {
            eprintln!("Building logger config failed: {err}");
            return Err(std::process::ExitCode::from(42));
        }
    };

    match log4rs::init_config(config) {
        Ok(handle) => Ok(handle),
        Err(err) => {
            eprintln!("Initializing logger failed: {err}");
            Err(std::process::ExitCode::from(43))
        }
    }
}
