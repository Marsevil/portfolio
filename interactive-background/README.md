# Build WASM

1. cargo build --target wasm32-unknown-unknown
2. wasm-bindgen --target web --out-dir dist ./target/wasm32-unknown-unknown/{debug/release}/interactive-background.wasm

# Dev env

The `index.html` file is here to provide a quick view.
To serve the file you can run :

```
deno run --allow-net --allow-read --allow-sys jsr:@std/http/file-server
```

# Credit

Portal: Companion Cube by Prateek Karajgikar [CC-BY] via Poly Pizza
