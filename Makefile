BIN = target/debug/tacky

binary:
	cargo build

test_1: binary
	writing-a-c-compiler-tests/test_compiler $(BIN) --chapter 1


.PHONY: install
install:
	cargo install --path app --force
