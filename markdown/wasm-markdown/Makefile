build: src/lib.rs
	cargo +nightly build --target=wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_markdown.wasm --out-dir build
