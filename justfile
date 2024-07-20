set shell := ["bash", "-c"]

clean:
    cargo clean

build:
    cargo build

build-release:
    cargo build --release

lint: add-clippy
	cargo clippy

fmt: add-fmt
	cargo fmt

run:
    cargo run

test:
	cargo test

add-clippy:
    rustup component add clippy 2> /dev/null

add-fmt:
    rustup component add rustfmt 2> /dev/null

default:
    just --list

changelog:
    git cliff -o CHANGELOG.md
