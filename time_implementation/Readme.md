## Building Rust Contract

We can build the contract using rustc:

```bash
 cargo build --target wasm32-unknown-unknown --release
```

## Deploying Rust Contract

```bash
near dev-deploy "wasmFile Path"
```
