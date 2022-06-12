default: format lint build e2e-test

format:
	cargo fmt

lint:
	cargo clippy

build:
	cargo build --release

test:
	bash ./tests/run.sh

# It is important that e2e tests are run serially on a single thread
e2e-test:
	cargo test

install:
	cargo install --path .
