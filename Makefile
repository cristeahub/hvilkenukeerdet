all: compile

.PHONY: compile
compile:
	cargo build

.PHONY: run
run:
	cargo run

.PHONY: release
release:
	cargo build --release

.PHONY: install
install:
	cp release/uke /usr/local/bin/
