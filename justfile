# Run the institute CLI: `just hp spark "an idea"`
hp *args:
    cargo run -q -p hp -- {{args}}

build:
    cargo run -q -p hp -- build

serve:
    cargo run -q -p hp -- serve

test:
    cargo test --workspace

check:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace

# Put `hp` on your PATH
install:
    cargo install --path labs/machinery
