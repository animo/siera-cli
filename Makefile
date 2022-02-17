all: clippy install

clippy:
	cargo clippy --no-deps

build:
	cargo build

run:
	cargo run

docs:
	cargo doc --open

install: clippy build
	cargo install --path .