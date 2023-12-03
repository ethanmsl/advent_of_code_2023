# Day 01 Notes (README) for Advent of Code 2023

## Notable Takeaways

- Lack of natural orderability of patterns in linear spaces.
- Regex DFAs are uncommon in practice. (partly an API and gen. knowledge issue and partly due to many practical regex considerations, including compilation time, size, and probably many perf issues in execution level matching algorithms)

## Questions

- [x] regex defined DFAs on stack?
- [] **regex-automata** crate allows this; bin to serialize and `include_bytes!()` to roll into main binary.
- [x] DFA construction macros and efficiency?
  - [] some interesting options; noted elsewhere
- [] Impact of ASCII restrictions on comparisons and Performance?
- [] Impact of direct byte comparisons on Performance?

## Rust Syntax Heroes

- `.extend()`
  - Extends a collection by an iterator.

## Rust Crate Heroes

- n/a

## PERFs

The questions were a natural fit for DFA implemented regex. (up to my ignorance of various instruction set level optimizations; at the least.)
However, looking a many regex crates I did not find what I was looking for.
I just wanted to, for example, run 9 automata in 'parallel' (literally or figuratively) and take first return. (front and reverse from back).

_Excitingly_, I found a crate that seems like it can do that. Or at least readily set up something similar.

Evidentally the regular regex crate is meant to paper over the details needed for those sorts of implementations. (Partly becuase they're balancing a lot of broader perf concerns.)
The **regex-automata** crate does allow for it. (Though it is intended to be a less friendly API.)

It _also_ alows precompilation and serialization of automata/regex patterns.
So I would define some binary file. Run it. Have the regex code saves somewhere. then do an `include_bytes` like one would `include_string`. And be able to roll it into binary that way.

### Pre-compile the Regex to a DFA:

```rust

use regex_automata::dense::DenseDFA;
use std::fs::File;
use std::io::Write;

fn main() {
    let dfa = DenseDFA::new("(\d+) Blue").unwrap();
    let serialized = bincode::serialize(&dfa).unwrap();

    let mut file = File::create("dfa.bin").unwrap();
    file.write_all(&serialized).unwrap();
}
```

### Include the Pre-compiled DFA in (another) Binary:

```rust

    use regex_automata::dense::DenseDFA;
    use regex_automata::DFA;

    fn main() {
        let dfa_bytes = include_bytes!("../dfa.bin");
        let dfa: DenseDFA<&[u8], u32> = bincode::deserialize(dfa_bytes).unwrap();

        let text = "123 Blue";
        if dfa.is_match(text.as_bytes()) {
            println!("Match found!");
        }
    }
```

From a bit of reading in the regex-automata docs `Dense` will be compilation long (and memory large), but speed short. For the tiny, simple patterns we're looking for that should be fine.

(I'm curious about mixing DFAs and their memory representation -- e.g. shared states for SIX & SIXTEEN)
By default nothing of the sort seems to occur. Naturally enough. THough this is in principle computable. I'm also _very_ curious to start looking at the perf tradeoffs.

## GENERALIZATIONs

- Natural generalizations of this have inherrent performance problems as there's no 'natural' ordering of patterns of varying lengths whose extents can overlap in multiple ways. e.g. `SIX` & `SIXTEEN` (and `SIXTY` & `SIXTEEN`). Or `AAA-BBB` & `BBB-CCC`.
  - Finding logical graphs who map cleanly to machine action graphs is an interesting problem.
