.PHONY: tests

create-env:
	python3 -m venv .venv

install:
	maturin develop

tests:
	cargo test

docker-build:
	docker run --rm -v $$(pwd):/io ghcr.io/pyo3/maturin:v1.7.0 build --release --universal2 -o dist

