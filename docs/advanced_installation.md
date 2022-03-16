# Advanced installation

It is possible to build and use the latest Aries CLI by following one of these
methods. Both require that you have `rust` and `cargo` ready
to go ([see Rust's instructions](https://www.rust-lang.org/learn/get-started)).

**Note:** Due to the variability between operating systems we cannot guarantee
that you will not run into an issue. We encourage you to open an issue on this
repository.

## Cargo

```sh
cargo install aries-cli
```

## Git

```sh
git clone https://github.com/animo/aries-cli.git
cd aries-cli
cargo install --path .
```