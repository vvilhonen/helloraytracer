pub fn install_logging() {
    ::std::env::set_var(
        "RUST_LOG",
        match ::std::env::var("RUST_LOG") {
            Ok(val) => val,
            _ => "info".to_owned(),
        },
    );
    pretty_env_logger::init_timed();
}
