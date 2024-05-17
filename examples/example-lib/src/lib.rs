#[no_mangle]
pub fn run(logger: tracing_shared::SharedLogger) {
    tracing_shared::setup_shared_logger(logger);

    println!("dylib println!");
    tracing::info!("dylib tracing::info!");
    #[cfg(feature = "log")]
    log::info!("dylib log::info!");
}
