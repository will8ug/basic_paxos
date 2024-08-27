test:
	cargo test --verbose

build:
	cargo build --verbose

run:
	cargo run

clean:
	cargo clean
	rm -f default_*.profraw

all: clean build
