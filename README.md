# rustc-less-ignore-debug

A utility for attempting to reduce the number of `// ignore-debug` tests, by
either removing the directively completely or use `// -Cdebug-assertions = no`
instead.

## Configuration file

You'll need to specify which directories for the tool to work on in `config.toml` generated in
the same directory as the executable. You can run e.g.

```rs
cargo run -- generate-config
```

to generate a default config that you can edit.
