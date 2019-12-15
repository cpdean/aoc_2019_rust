trying to get a multicrate thing working for rust.

```
cargo test --all
```

```
# for running old days, until this gets backported to the workspace format
cargo run --example day01
RUST_BACKTRACE=1 cargo run --example day06
cargo run --example day09
```

```
cargo run --bin day11
cargo test -p day11
```
