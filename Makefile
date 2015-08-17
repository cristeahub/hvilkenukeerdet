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
install: release
	cp target/release/uke /usr/local/bin/
