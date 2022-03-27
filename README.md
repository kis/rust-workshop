RUST_LOG=debug cargo run -p ontour-cli  
cargo run -p ontour-cli -- --help  
./target/debug/cli --help  
cargo run -p ontour-cli -- crates/ontour/tests/test.wasm hello "Potter"  
cargo run -p ontour-cli -- crates/ontour/tests/test.wasm hello crates/ontour-cli/hello.json  
