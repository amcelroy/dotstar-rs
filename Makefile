wasm:
	cargo rustc --features wasm --crate-type=cdylib
	wasm-pack build --features wasm

test:
	cargo test --lib