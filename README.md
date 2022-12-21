# Provenance Rust

The home of all rust resources for the Provenance ecosystem.

## Crates

| Name                 | Description                 | crates.io | CI Build |
|----------------------|-----------------------------|-----------|----------|
| [`provenance‑sdk‑proto`] | Proto definitions  | [![crates.io][provenance-sdk-proto-crate-img]][provenance-crate-link] | [![CI][provenance-sdk-proto-ci-img]][provenance-sdk-proto-ci-link] |

## Rebuilding Proto files

The `proto-build` crate is responsible for updating the submodule and rebuilding the proto files
which are used by the other crates. Update the version in [main.rs](proto-build/src/main.rs) and run

```
cargo run
```

[//]: # "crates"

[`provenance‑sdk‑proto`]: https://github.com/provenance-io/provenance-rs/tree/main/provenance-sdk-proto

[//]: # "badges"

[provenance-sdk-proto-crate-img]: https://img.shields.io/crates/v/provenance-sdk-proto.svg?logo=rust
[provenance-crate-link]: https://crates.io/crates/provenance-sdk-proto
[provenance-sdk-proto-ci-img]: https://github.com/provenance-io/provenance-rs/workflows/build/badge.svg
[provenance-sdk-proto-ci-link]: https://github.com/provenance-io/provenance-rs/actions/workflows/build.yml
