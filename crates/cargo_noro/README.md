# cargo-noro

Create, build, check and package a module for a
[Noro](https://github.com/NoroProject/noro-shared) instance.

```bash
cargo install cargo-noro

cargo noro new my-module     # a working module, or --bare for an empty one
cargo noro add event join    # scaffold event handler, route, task or migration
cargo noro ui                # launch local web UI testbed with mock window.__noroUi
cargo noro check             # the manifest, locale keys, migrations
cargo noro dev               # rebuild on every save
cargo noro package           # dist/my-module.noromod — this is what you upload
```

`check` reads the manifest with the **same types the master uses**, so an unknown
capability action, a locale key without its prefix or a mini-app pointing at a file that
is not there are refused here — before the upload rather than after it.

`cargo noro key new` makes a signing key. Signing is about the *update*: a package with
the same identifier and a different key is a different author, and the master refuses it
rather than accepting it quietly.

**Documentation: <https://noroproject.github.io/noro-shared/reference/cli/>**

## Licence

MIT.
