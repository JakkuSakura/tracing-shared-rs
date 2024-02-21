# tracing-shared-rs

## Usage

```toml
[dependencies]
tracing-shared = "0.1"

```

```rust
// In main.rs
fn main() {
    // Build a logger from main program
    let logger = build_shared_logger();
    // Load the dll
    let dll = todo!();
    // Then pass the logger to the dll
    dll.setup_shared_logger(logger);
}
```