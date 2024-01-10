RUSTFLAGS="-Zlocation-detail=none" cargo build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target wasm32-unknown-unknown --profile=wasm

wasm-snip target/wasm32-unknown-unknown/release/game.wasm -o target/wasm32-unknown-unknown/wasm/game.wasm
wasm-opt -Oz -o target/wasm32-unknown-unknown/wasm/game.wasm target/wasm32-unknown-unknown/wasm/game.wasm
