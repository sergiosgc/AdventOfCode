default: one.input two.input

one.sample: target/debug/one
	cargo run --bin=one < one.sample.txt

one.input: target/debug/one
	cargo run --release --bin=one < one.input.txt

target/debug/one: Cargo.toml src/bin/one.rs
	cargo build

two.sample: target/debug/two
	cargo run --release --bin=two < two.sample.txt

two.input: target/debug/two
	cargo run --release --bin=two < two.input.txt

target/debug/two: Cargo.toml src/bin/two.rs
	cargo build

watch:
	(find -name \*.rs) | entr -acc make
