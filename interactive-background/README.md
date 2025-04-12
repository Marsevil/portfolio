# Build WASM

- `cargo make dbg` to build in debug
- `cargo make rel` to build in release

Generated bindings can be found in the `dist` directory.

# Dev dependencies

- `wasm-bindgen`
- `cargo-make` (optional, take a look at `Makefile.toml` to see command combination)

# Dev env

The `index.html` file is here to provide a quick view.
To serve the file you can run :

```
deno run --allow-net --allow-read --allow-sys jsr:@std/http/file-server
```

Or whatever to serve files

# Credit

- Portal: Companion Cube by Prateek Karajgikar [CC-BY] via Poly Pizza
- Oculus Controller by NekoCat [CC-BY] via Poly Pizza
- Videogame Controller by Poly by Google [CC-BY] via Poly Pizza
- Electric guitar by Poly by Google [CC-BY] via Poly Pizza
- Guitar Amp by Poly by Google [CC-BY] via Poly Pizza
- Tree by ParfaitUwU [CC-BY] via Poly Pizza
- Mushroom by jeremy [CC-BY] via Poly Pizza
