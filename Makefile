toolchain:
	./scripts/init.sh

build:
	cargo build

install:
	cargo install --force --path bin/json
	cargo install --force --path bin/jtime

init: toolchain
