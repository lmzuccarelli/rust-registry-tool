.PHONY: all test build clean

all: clean test build

LEVEL ?= "info"
TEST ?= ""
DIFF ?= false
DOCKER_DESTINATION ?= "docker://127.0.0.1:5000/test"

build-debug: 
	cargo build

build:
	cargo build --release

test: clean-tests
	CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test  -- --nocapture --test-threads=1

test-by-name: clean-tests
	CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test $(TEST) -- --nocapture --test-threads=1

cover:
	grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" --ignore "src/main.rs" --ignore "src/api/*" --ignore "src/mirror/upload.rs" -o target/coverage/html
	cp target/coverage/html/html/badges/flat.svg assets/

run-d2m:
	cargo run -- --config imagesetconfig.yaml --diff-tar $(DIFF)  --loglevel $(LEVEL) --destination $(DOCKER_DESTINATION)

run-m2d:
	cargo run -- --config imagesetconfig.yaml --diff-tar $(DIFF)  --loglevel $(LEVEL) --destination file://

clean-all:
	rm -rf cargo-test*
	cargo clean
	rm -rf ./target/debug

clean-tests:
	rm -rf cargo-test*
