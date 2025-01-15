wasm:
	wasm-pack build --features wasm

test-macos:
	cargo test --target aarch64-apple-darwin