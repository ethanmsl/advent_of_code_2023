# Day 05 Notes (README) for Advent of Code 2023

## Notable Takeaways

- Remarkably hard to find a general split-like iterator (splitting on match of arbitrary logic). And the grouping style iterators in itertools appeared very inefficient.
- Excited that the naive approach took some time. Buoyed that there will be some fun room for looking at performance and allotments and related good stuff -- the allure of measuring which brought me to the contest this year.
- _Interesting_ example of a case where the "natural" representation of objects would result in very inefficent code. (I should test though! I don't know what the compiler might do to millions upon millions of heap strings doing nothing at all - lol!)
  - Will be interesting to think more about representing dynamics and problem vs possibiliteis of the objects in question.

## Questions

- How to get dynamic statics. e.g. to use a string of text that may be repeated in many cases to be useable as a label and have a reference passed around? (Part 2 did not end up making that necessary, but the question still stands.)

## Rust Syntax Heroes

- lol, `.split()`. I spent quite a bit of time looking at `.batching()` and `.group_by()` in the itertools crate. And quite unhappy with the sorts of allocations they seemed to require. I looked for general split style iterators, even using the name "split" without thinking about that strin operation. -- that said, I still haven't found, but want, an iterator that splits on a match of arbitrary logic. I'm not sure why that was so hard to find...

## Rust Crate Heroes

- none (though perhaps Rayon with playful refactor)

## PERFs

- Obv. (and probably "right") perf is to operate on ranges rather than numbers. Splitting and recombining ranges as you go. So you'd have to do some range math and change the logic to do all ranges together on each step (to allow potential recombination perf).
- Easy and fun: Rayon crate with `par_iter`. Not likely to be more than 4-8x improvement (vs the above which I'd guess would be 100x+), but very quick and easy and I've wanted to try the crate.

## GENERALIZATIONs

- Split style iterator with arbitrary matching logic.
