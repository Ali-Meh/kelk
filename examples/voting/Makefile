test: 
	cargo +nightly test --target wasm32-unknown-unknown -Z unstable-options

build:
	cargo +nightly build --target wasm32-unknown-unknown --release -Z unstable-options --out-dir ./wasm
	wasm-opt -Os -o wasm/calculator.wasm wasm/calculator.wasm

release:
	cargo build --target wasm32-unknown-unknown --release