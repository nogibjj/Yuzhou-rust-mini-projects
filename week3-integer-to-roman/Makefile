rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

# run:
# 	cargo run

release-arm:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy

run:
	cargo lambda invoke --remote \
		--data-ascii '{"integer": "112"}' \
		--output-format json \
		week3

all: format lint run
