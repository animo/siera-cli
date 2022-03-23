default: lint build test

lint:
	cargo clippy

build:
	cargo build --release

test: 
	sh ./tests/run.sh

install:
	cargo install --path ./cli
