# noro-sdk-macros

The attributes a [Noro](https://github.com/NoroProject/noro-shared) module is written
with. Use them through [`noro-sdk`](https://crates.io/crates/noro-sdk) rather than
directly.

```rust
#[noro::module]
impl Shop {
    #[event]                       // the event name comes from the argument type
    fn on_join(e: PlayerJoined) -> Result<()> { Ok(()) }

    #[event]                       // &mut declares a Pre handler: it can cancel or change
    fn gate(e: &mut UserPreRename) -> Result<()> { Ok(()) }

    #[route(GET, "/top")]
    fn top(req: HttpRequest) -> Result<Vec<Row>> { Ok(vec![]) }

    #[task("1h")]
    fn payout() -> Result<()> { Ok(()) }
}
```

No handler name is written as a string anywhere, and the event name is read from the
type — subscribing to one event while accepting another's struct does not compile.

**Documentation: <https://noroproject.github.io/noro-shared/>**

## Licence

MIT.
