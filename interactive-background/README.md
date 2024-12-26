# Build WASM

1. cargo build --target wasm32-unknown-unknown
2. wasm-bindgen --target web --out-dir dist ./target/wasm32-unknown-unknown/{debug/release}/interactive-background.wasm

# Credit

Portal: Companion Cube by Prateek Karajgikar [CC-BY] via Poly Pizza
