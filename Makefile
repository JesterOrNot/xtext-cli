install:
	cargo install --path . --force
.PHONY: install

target/release/xtext: src/*
	cargo build --release
