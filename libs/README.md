# libs

Contains modified versions of third-party libraries used by the project.

- [ux2](https://crates.io/crates/ux2/) - Library for non-standard integer types, modified to add `no_std` support.
- [wasmparser-nostd](https://crates.io/crates/wasmparser-nostd/) - Library for parsing WebAssembly binaries, modified to replace `Arc` with `Rc` since the Pico has no atomics.