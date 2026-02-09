GIT_HOME = $(git rev-parse --show-toplevel)
CARGO_BIN_PATH = rust/target/debug/rust

build: link

rust: rust/src/main.rs
	cargo build --manifest-path rust/Cargo.toml
	mv $(CARGO_BIN_PATH) build/rust.o

c: c/main.c
	clang -c c/main.c -o build/main.o

link: rust c build/main.o build/rust.o
	lld -o build/main build/main.o build/rust.o