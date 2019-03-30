This repo is for reproducing a bug in [Juniper][].

Run `cargo test` to reproduce the error:

```
$ cargo test
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running target/debug/deps/juniper_bug_reproduction-3606a7a034be58af

running 2 tests
test test::without_fragment ... ok
test test::with_fragment ... FAILED

failures:

---- test::with_fragment stdout ----
thread 'test::with_fragment' panicked at 'not yet implemented', /Users/davidpdrsn/.cargo/registry/src/github.com-1ecc6299db9ec823/juniper-0.11.1/src/executor/look_ahead.rs:267:18
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


failures:
    test::with_fragment

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```

Tested on `rustc 1.33.0 (2aa4c46cf 2019-02-28)`

[Juniper]: https://github.com/graphql-rust/juniper
