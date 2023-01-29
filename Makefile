rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cd week1 && cargo fmt --quiet && cd ../

lint:
	cd week1 && cargo clippy --quiet && cd ../

test:
	cd week1 && cargo test --quiet && cd ../

run:
	cd week1 && cargo run && cd ../

release:
	cd week1 && cargo build --release && cd ../

all: format lint test run
