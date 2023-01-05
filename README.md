# type-rs
This should be a cli to practice touch typing using lines of rust code. Maybe eventually to be extended to other languages.

# run
It will get lines of code from `$HOME/.cargo/registry/src/**` so the `HOME` env variable needs to be set.
```
cargo run -q
```

## TODO
- [x] Open random file and print random line  
- [x] Change color as you type
- [x] Results after each line
- [x] Results at the end
- [ ] Choose line or block
- [ ] Separate the code fetching to a different crate
- [ ] Add clap
- [x] Add instructions
- [ ] Need to backspace to fix
- [x] Esc key to stop
- [ ] Remove all the unwrap()
- [ ] Add tests
- [ ] Publish binary crate
- [ ] ...
