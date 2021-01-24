toolchain:
	./scripts/init.sh

build:
	cargo build

install:
	cargo install --force --path bin/*

init: toolchain
