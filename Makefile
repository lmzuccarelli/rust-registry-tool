.PHONY: all build clean-all

all: clean-all build

build-debug: 
	cargo build

build:
	cargo build --release

clean-all:
	cargo clean
	rm -rf ./target/debug
