# tracing-shared-rs

Share a logger between a dylib/cdylib and the main binary

## Usage

```toml
[dependencies]
tracing-shared = "0.1"

```

checkout examples/example.rs

### cdylib's case

```rust
fn main() {
    let dylib = unsafe { libloading::Library::new(dylib) }.expect("error loading dylib");
    let setup_logger: FnSetupLogger = unsafe { *dylib.get(b"setup_shared_logger_ref").unwrap() };
    let run: FnRun = unsafe { *dylib.get(b"run").unwrap() };
    let logger = SharedLogger::new();
    setup_logger(&logger);
    run("cdylib")
}
```

### cdylib's case

```rust
use tracing_shared::SharedLogger;

fn main() {
    let dylib = unsafe { libloading::Library::new(dylib) }.expect("error loading dylib");
    let setup_logger: FnSetupLogger = unsafe { *dylib.get(b"setup_shared_logger_ref").unwrap() };
    let run: FnRun = unsafe { *dylib.get(b"run").unwrap() };
    let logger = SharedLogger::new();
    setup_logger(&logger);
    run("cdylib")
}
```

### dylib's case

```rust
use tracing_shared::SharedLogger;

fn main() {
    let logger = SharedLogger::new();
    example_lib::setup_shared_logger_ref(&logger);
    example_lib::run("dylib");
}
```