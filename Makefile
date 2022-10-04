default: format lint build e2e-test

format:
	cargo fmt

lint:
	cargo clippy

lint-strict:
	cargo clippy -- -D clippy::all -D clippy::pedantic -D clippy::nursery -D clippy::missing_docs_in_private_items -A clippy::struct_excessive_bools -A clippy::module_name_repetitions -A clippy::only_used_in_recursion

build:
	cargo build --release

test:
	bash ./tests/run.sh

# It is important that e2e tests are run serially on a single thread
e2e-test:
	cargo test

install:
	cargo install --path .
