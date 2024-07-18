.PHONY: tests

create-env:
	python3 -m venv .venv

tests:
	cargo test

build:
	cargo build