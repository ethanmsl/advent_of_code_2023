# Day 02 Notes (README) for Advent of Code 2023

## Notable Takeaways

- lorem ipsum

## Questions

- const vs. static definition relative to macro compile time.
- const & static performance advantages

## Rust Syntax Heroes

- _Option<>_`.unwrap_or(_)`
  - great way to add local default to an Option
- `const`
  - from a readability perspective alone

## Rust Crate Heroes

- (standard) regex crate
  - _not_ for its use in the core logic, but for doing exploratory/ad-hoc input searches
    - e.g. due to some issue with feedback on problem I spent a long time attemptint to debug a totally working solution -- being able to quickly run regex on the large input space to look for and _rule out_ pathological examples was very helpful!

## PERFs

- Short Circuiting: .max |> cmp*to_ceiling should be (up to execution optimizations) much \_slower*, typically, than just running the cmp_to_ceiling and short-circuiting. Should be simple to implement and clean to read as well.

## GENERALIZATIONs

- lorem ipsum
