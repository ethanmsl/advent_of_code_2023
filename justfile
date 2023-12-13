# default recipe to display help information
default:
    @just --list

#TODO: failure, fix children first
work day_digits part_digit:
    cargo watch -x "check -p day-{{day_digits}}" -s "just test part{{part_digit}} -p day-{{day_digits}}" -s "just lint day-{{day_digits}}" -s "just bench day-{{day_digits}} part{{part_digit}}" -s "just flamegraph day-{{day_digits}} part{{part_digit}}"

www-watch:
    RUST_LOG=info cargo +nightly leptos watch --project www

www-build:
    cargo +nightly leptos build --project www --release

# TODO: failure; unkown reason
lint day_digits:
    cargo clippy -p day-{{day_digits}}

# uses novel `cargo nextest` and a separate docs test
test day_digits part_digit +LEVEL="debug":
    RUST_LOG={{LEVEL}} cargo nextest run -p day-{{day_digits}} part{{part_digit}} --no-capture
    RUST_LOG={{LEVEL}} cargo test -p day-{{day_digits}} --doc

# uses traditional `cargo test`
test-trad day_digits part_digit:
    cargo test -p day-{{day_digits}} --lib part{{part_digit}}

# test part_digit +FLAGS='-p day-01':
#     cargo nextest run {{FLAGS}} part{{part_digit}}

# Bench-all days and parts.
bench-all:
    cargo bench --quiet > benchmarks.txt

# Quick bench a specific day & part.
bench day_digits part_digit:
    cargo bench --bench day-{{day_digits}} part{{part_digit}} >> day-{{day_digits}}_{{part_digit}}.bench.txt

# Clean up individual bench.txt files in root.
clean-benches:
    rm day-*_*.bench.txt

# TODO: fix writing
flamegraph day_digits part_digit:
    cargo flamegraph --profile flamegraph --root --package day-{{day_digits}} --bin part{{part_digit}} -o flamegraphs/day-{{day_digits}}--part{{part_digit}}.svg

# Heap profiling of specific day & part.
dhat day_digits part_digit:
    cargo run --profile dhat --features dhat-heap --package day-{{day_digits}} --bin part{{part_digit}}

# Create day-specific crate
create day_digits:
    cargo generate --path ./template__daily_problem --name day-{{day_digits}}
