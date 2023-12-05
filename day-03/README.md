# Day 03 Notes (README) for Advent of Code 2023

## Notable Takeaways

- "Half-Correct" is probably 'most wrong' in many situations. e.g. moving to u64 to model data rather than properly working on a representation of actual boundries results in implementation awkwardness without correctness and likely a fair bit of lost work on a future refactor.
  - e.g. here using either iMN to ensure no wrap around, working on a boundry data type, or quick hack involving modifying input with buffer (should be easy to remove ... though it doesn't track to issues that need to be changed with it ... so maybe another example of bad...)
- Wrapper types exposing their internal fields is probably a good place to start.
  - quick and clean, and easy to refactor
  - deref is tempting, but (with what seems like good reason) considered an anti-pattern when used on non-pointers as it creates a lot of potential ambiguity in the code.
  - a simple wraper.hmap by contrast is quite clear and not difficult to work with
  - and can be easily refactored if the field is placed as private later

## Questions

- Best derives and crates for ergonomic use of wrapper types?
- impact of reusing an iterator (copy vs clone and pointer to vs owner of data)
- good crates for working with n-dimensional grids and data?
- (particularly, e.g., for automata with concepts of locality)
- how to shut warnings during tests while mid-work / how and whether to change workstyle to aovid them.
- Turning off mixed character set warnings -- to allow use of greek letters in code (e.g. `it_δ` for `it_<delta>`)
- whether and how to change case warnings for non-standard letters (e.g. `it_δ` vs `it_Δ`)

## Rust Syntax Heroes

- itertools > `.cartesian_product`

## Rust Crate Heroes

- none; though the itertools `.cartesian_product` is quite nice.

## PERFs

- Regex roll into binary, vs computing live.
- Searching for both patterns simultaneously, to slightly reduce searched space (since non-overlapping regions)
- working with ascii
- working on size ranges specific to input
  - also a safety and ergonomics issue
- pointers for data in hashmap where multiple keys point to value
  - (using once_cell, for example) <-- need to check both indirection cost and once_cell deets

## GENERALIZATIONs

- Customizable Hashable IDs for custom structs (?)
  - allows hashset instead of hashmap, since data for mapping is part of value itself
