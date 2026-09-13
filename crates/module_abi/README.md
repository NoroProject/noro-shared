# noro-module-abi

The boundary between a [Noro](https://github.com/NoroProject/noro-shared) master and a
module: event payloads, entity projections, the manifest and its validation.

You rarely depend on this directly — [`noro-sdk`](https://crates.io/crates/noro-sdk)
re-exports what an author needs. It is separate because two parties read the same
manifest: `cargo noro` when building a package and the master when installing one. A
second copy of those rules would let an author build a package the master silently
refuses.

The `signing` feature adds package signature verification. It is off by default: the
crate is linked into every module, and ed25519 with sha2 weigh real kilobytes in wasm
that a module never uses.

**Documentation: <https://noroproject.github.io/noro-shared/>**

## Licence

MIT.
