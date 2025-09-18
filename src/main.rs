fn main() -> std::process::ExitCode {
    let runtime: tokio::runtime::Runtime = match tokio::runtime::Builder::new_current_thread().build() {
        Ok(n) => n,
        Err(err) => {
            eprintln!("{err}");
            return std::process::ExitCode::from(42);
        }
    };

    std::process::ExitCode::SUCCESS
}
