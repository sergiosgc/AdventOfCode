default: one.input two.input

one.sample: target/debug/one
	cargo run --bin=one < one.sample.txt

one.input: target/release/one
	cargo run --release --bin=one < one.input.txt

target/debug/one: Cargo.toml src/bin/one.rs
	cargo build

target/release/one: Cargo.toml src/bin/one.rs
	cargo build --release

two.sample: target/debug/two
	cargo run --bin=two < two.sample.txt

two.input: target/release/two
	cargo run --release --bin=two < two.input.txt

target/debug/two: Cargo.toml src/bin/two.rs
	cargo build

target/release/two: Cargo.toml src/bin/two.rs
	cargo build --release

watch:
	(find -name \*.rs) | entr -acc make
