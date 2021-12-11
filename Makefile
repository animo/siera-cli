clippy:
	cargo clippy

build:
	cargo build

run:
	cargo run

install: clippy build
	cargo install --path .
