# Provenance Rust

The home of all rust resources for the Provenance ecosystem.

## Crates

TODO after release

## Rebuilding Proto files

The `proto-build` crate is responsible for updating the submodule and rebuilding the proto files
which are used by the other crates. Update the version in [main.rs](proto-build/src/main.rs) and run

```
cargo run
```
