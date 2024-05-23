use example_lib::exports::{FnRun, FnSetupLogger};
use std::time::Duration;
use std::{io::Write, path::PathBuf};
use tracing_shared::SharedLogger;

fn main() {
    // build example-lib
    let cdylib = build_dylib();

    // set tracing logger
    tracing_subscriber::FmtSubscriber::builder()
        .without_time()
        .init();

    // log in the normal program
    println!("program println!");
    tracing::info!("program tracing::info!");
    // log was supported in tracing_subscriber
    #[cfg(feature = "log")]
    log::info!("program log::info!");
    // test if tokio task spawns properly
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            // log in the rust dylib
            run_dylib();
            // log in the cdylib, see `example-lib/src/lib.rs`
            run_cdylib(cdylib);
            // wait for spawned tasks to finish
            tokio::time::sleep(Duration::from_secs(1)).await
        })
}

fn run_dylib() {
    let logger = SharedLogger::new();
    example_lib::setup_shared_logger_ref(&logger);
    example_lib::run("dylib");
}
fn run_cdylib(dylib: PathBuf) {
    let dylib = unsafe { libloading::Library::new(dylib) }.expect("error loading dylib");
    let setup_logger: FnSetupLogger = unsafe { *dylib.get(b"setup_shared_logger_ref").unwrap() };
    let run: FnRun = unsafe { *dylib.get(b"run").unwrap() };
    let logger = SharedLogger::new();
    setup_logger(&logger);
    run("cdylib")
}
fn build_dylib() -> PathBuf {
    print!("building `example-lib`...");
    std::io::stdout().flush().unwrap();

    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("build")
        .arg("--manifest-path")
        .arg("examples/example-lib/Cargo.toml")
        .arg("--message-format")
        .arg("json");

    #[cfg(feature = "log")]
    cmd.arg("--features=log");

    let output = cmd.output().unwrap();
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("{error}");
        panic!("dylib build failed");
    }

    let output = String::from_utf8_lossy(&output.stdout);

    for line in output.lines().rev() {
        if line.starts_with(r#"{"reason":"compiler-artifact""#) {
            let files_start = r#""filenames":[""#;
            let i = line.find(files_start).unwrap();
            let line = &line[i + files_start.len()..];
            let i = line.find('"').unwrap();
            let dylib = &line[..i];

            println!(" done");

            return PathBuf::from(dylib);
        }
    }
    panic!("failed to find get dylib output");
}
