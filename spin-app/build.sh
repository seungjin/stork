cd ../stork;
cargo component build --release;

cd ../spin-app;
cargo component build --release;

wasm-tools compose -d ../stork/target/wasm32-wasi/release/stork.wasm ./target/wasm32-wasi/release/spin_app.wasm -o service.wasm

