.PHONY: tests

create-env:
	python3 -m venv .venv
	pip install pytest

install:
	maturin develop

tests:
	cargo test

docker-build:
	docker run --rm -v $$(pwd):/io ghcr.io/pyo3/maturin:v1.7.0 build --target aarch64-apple-darwin --release --out dist --find-interpreter

