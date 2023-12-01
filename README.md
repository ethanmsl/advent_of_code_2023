# Advent of Code 2023

Motivation: learning better profiling techniques from Biscardi 2023 series
Sub-Motivation: general comparision to various idiomatic practices

For overview of Setup see the "My Setup" section of this youtube video: [How to set up Rust for Advent of Code - Chris Biscardi](https://youtu.be/fEQv-cqzbPg?si=0_AqsxhTAQVKA5n9&t=431)

## To create new daily file:

`just create XY`

NOTE: syntax is important as justfile uses this when executing other commands
NOTE: justfile rewritten to take 2-digit numbers for day and a 1-digit number for part. (instead of "day-XY" and "partz")

TODO: have the `todo!(D)ay __)` template text auto populated on creation

### Accessories used, for install Needs:

```shell
cargo install cargo-watch
cargo install cargo-generate
cargo install flamegraph
cargo install cargo-expand
cargo install cargo-show-asm
cargo install cargo-nextest --locked
cargo install just
```
