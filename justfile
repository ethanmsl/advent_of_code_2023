#TODO: failure, fix children first
work day_digits part_digit:
    cargo watch -x "check -p day-{{day_digits}}" -s "just test part{{part_digit}} -p day-{{day_digits}}" -s "just lint day-{{day_digits}}" -s "just bench day-{{day_digits}} part{{part_digit}}" -s "just flamegraph day-{{day_digits}} part{{part_digit}}"
www-watch:
   RUST_LOG=info cargo +nightly leptos watch --project www
www-build:
   cargo +nightly leptos build --project www --release
# TODO: failure; unkown reason
lint day_digits:
    clippy-tracing --action check --exclude target --exclude benches --exclude www
    cargo clippy -p day-{{day_digits}}
# TODO: "day-01" hard coded, also unclear goal
test day_digits part_digit:
    cargo nextest run -p day-{{day_digits}} part{{part_digit}}
# test part_digit +FLAGS='-p day-01':
#     cargo nextest run {{FLAGS}} part{{part_digit}}
bench-all:
    cargo bench -q > benchmarks.txt
bench day_digits part_digit:
    cargo bench --bench day-{{day_digits}} partpart{{part_digit}} >> day-{{day_digits}}.bench.txt
# TODO: fix writing
flamegraph day_digits part_digit:
    cargo flamegraph --profile flamegraph --root --package day-{{day_digits}} --bin part{{part_digit}} -o flamegraphs/day-{{day_digits}}--part{{part_digit}}.svg
dhat day_digits part_digit:
    cargo run --profile dhat --features dhat-heap --package day-{{day_digits}} --bin part{{part_digit}}
create day_digits:
    cargo generate --path ./template__daily_problem --name day-{{day_digits}}
