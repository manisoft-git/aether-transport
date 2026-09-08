.PHONY: all build check test bench clean fuzz miri kani doc paper netem-bench

all: build test

build:
	cargo build --workspace --release

check:
	cargo check --workspace --all-targets
	cargo clippy --workspace --all-targets -- -D warnings

test:
	cargo test --workspace

bench:
	cargo bench --workspace

miri:
	cargo miri test -p aether-math -p aether-wire

kani:
	cargo kani --dir kani/

doc:
	cargo doc --workspace --no-deps --open

paper:
	bash paper/compile_paper.sh

netem-bench:
	bash scripts/netem_benchmark.sh

clean:
	cargo clean
	rm -rf paper/*.pdf paper/*.aux paper/*.log paper/*.bbl paper/*.blg
