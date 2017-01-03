# rust-clock
A looping command-line clock written in rust.

# Dependencies
This makes use of the [Time](https://doc.rust-lang.org/time/time/index.html) crate.

# How to build your binary
- Development (unoptimized)
```bash
cargo build
```
- Release (optimized)
```bash
cargo build --release
```

# Example output
```bash
user@~/rust_clock/target/release$ ./rust_clock
six seventeen in the evening
six eighteen in the evening
```

# To Do
- ~~Make the looping cleaner so it just doesn't continue line by line, printing same time.~~
- Clear the screen when time changes.
