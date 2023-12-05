//! Objects for Day-03
use derive_more::{AsMut, AsRef, Constructor, IntoIterator};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashSet;
use tracing::info;

/// A simple vec of relevant info for each number
#[derive(AsRef, AsMut, IntoIterator, Debug)]
pub struct NumberRegister {
        vec: Vec<NumberInfo>,
}

impl NumberRegister {
        /// Constructor
        pub fn new() -> Self {
                Self { vec: Vec::new() }
        }

        /// Register a number and its info
        pub fn register_numbers(&mut self, row: i64, raw_line: &str) {
                static RE_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

                RE_NUMBER.find_iter(raw_line).for_each(|m| {
                        let (h_start, _) = (m.start() as i64, m.end());
                        let len = m.len() as i64;
                        let val = m.as_str().parse::<u64>().expect("parse failure");
                        let start_loc = (row, h_start);
                        let numinfo = NumberInfo::new(start_loc, len, val);
                        self.vec.push(numinfo);
                });
        }
}

/// Info needed to work with each number
/// start_Location as a unique id and seed for location values
/// len to calculate all values touched by number
#[derive(Constructor, AsRef, AsMut, Debug)]
pub struct NumberInfo {
        start_loc: (i64, i64),
        len: i64,
        val: u64,
}

impl PartialEq for NumberInfo {
        /// Equality here is *only* based on the location value,
        fn eq(&self, other: &Self) -> bool {
                self.id() == other.id()
        }
}
impl Eq for NumberInfo {}

impl NumberInfo {
        pub fn id(&self) -> (i64, i64) {
                self.start_loc
        }

        /// Locations that a number covers (i.e. position of each digit)
        pub fn locations(&self) -> Vec<(i64, i64)> {
                let (row, col) = self.start_loc;
                let mut out = Vec::new();
                for rδ in 0..self.len {
                        for cδ in 0..self.len {
                                out.push((row, col + cδ));
                        }
                }
                out
        }

        pub fn val(&self) -> u64 {
                self.val
        }
}

/// All points touched by Special Chars
#[derive(AsRef, AsMut, IntoIterator, Debug)]
pub struct SpecialAdjacenciesRegister {
        set: HashSet<(i64, i64)>,
}

impl SpecialAdjacenciesRegister {
        /// Constructor
        pub fn new() -> Self {
                Self {
                        set: HashSet::new(),
                }
        }

        /// Register all adjacencies (inclusive of number itself)
        pub fn register_special_adjacencies(&mut self, row: i64, raw_line: &str) {
                // `[^.\d]` any char that's neither a literal `.` nor digit
                // (`.` is taken literally inside brackets, vs being an almost-any char normally)
                static RE_SPECIAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^.\d]").unwrap());
                RE_SPECIAL.find_iter(raw_line).for_each(|m| {
                        info!("m: {:?}", m);
                        let char_loc = m.start() as i64;
                        let new_adjacencies = self.calculate_adjacencies(row, char_loc);
                });
        }

        /// all the locations of a char +/-1
        /// WARNING: no bounds are checked
        /// For our current problems this is merely a minor perf issue at most (possibly a perf itself)
        /// As number digits cannot exist outside bounds
        /// NOTE: we are not allowing the char itself to be represented
        /// This is appropriate to our problem, but arguably unintuivie
        /// WARNING: we assume row lengths to be more than one less than i64::MAX
        /// TODO: consider ways of propogating constraints -- so that guards aren't being re-checked
        /// constantly locally..
        fn calculate_adjacencies(&mut self, row: i64, col: i64) {
                // to avoid recasting we use delta of + of our fix
                let it_δ = (0..=2)
                        .cartesian_product(0..=2)
                        .map(|(x, y)| (x - 1, y - 1));
                let it_adj = it_δ.map(|(rδ, cδ)| (rδ + row, cδ + col));
                // // TODO: check bounds
                // // for right now this is merely a PERF issue
                // if r < 0 || c < 0  || r > r_max || c > c_max {

                for p in it_adj {
                        self.set.insert(p);
                }
        }

        /// Checks if location as point set is in register
        /// (just skips wrapper to check hashset)
        pub fn contains(&self, loc: (i64, i64)) -> bool {
                self.set.contains(&loc)
        }
}
