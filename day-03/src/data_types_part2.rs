//! Objects for Day-03
use derive_more::{AsMut, AsRef, Constructor, IntoIterator};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use tracing::info;

/// A simple vec of relevant info for each number
#[derive(Debug)]
pub struct NumberRegister {
        // obv we could just use NumInfo.val (i64); throwing something on
        pub hmap: HashMap<(i64, i64), NumberInfo>,
}

impl NumberRegister {
        /// Constructor
        pub fn new() -> Self {
                Self {
                        hmap: HashMap::new(),
                }
        }

        /// Register a number and its info
        pub fn register_numbers(&mut self, row: i64, raw_line: &str) {
                static RE_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

                RE_NUMBER.find_iter(raw_line).for_each(|m| {
                        let (h_start, h_end) = (m.start() as i64, m.end() as i64);
                        let len = m.len() as i64;
                        let val = m.as_str().parse::<u64>().expect("parse failure");
                        let start_loc = (row, h_start);
                        let numinfo = NumberInfo::new(start_loc, len, val);
                        // add all locations number covers
                        for col in h_start..h_end {
                                self.hmap.insert((row, col), numinfo.clone());
                        }
                });
        }
        /// Checks if location as point set is in register
        /// (just skips wrapper to check hashset)
        pub fn contains(&self, loc: (i64, i64)) -> bool {
                self.hmap.contains_key(&loc)
        }
}

/// Info needed to work with each number
/// start_Location as a unique id and seed for location values
/// len to calculate all values touched by number
#[derive(Constructor, AsRef, AsMut, Debug, Clone)]
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

        pub fn val(&self) -> u64 {
                self.val
        }
}

/// All points touched by Special Chars
#[derive(AsRef, AsMut, IntoIterator, Debug)]
pub struct StarAndAdjacenciesRegister {
        pub hmap: HashMap<(i64, i64), Vec<(i64, i64)>>,
}

impl StarAndAdjacenciesRegister {
        /// Constructor
        pub fn new() -> Self {
                Self {
                        hmap: HashMap::new(),
                }
        }

        /// Register all adjacencies (inclusive of number itself)
        pub fn register_special_adjacencies(&mut self, row: i64, raw_line: &str) {
                // literal `*`
                static RE_SPECIAL: Lazy<Regex> = Lazy::new(|| Regex::new(r"[*]").unwrap());
                RE_SPECIAL.find_iter(raw_line).for_each(|m| {
                        info!("m: {:?}", m);
                        let col = m.start() as i64;
                        let new_adjacencies = self.calculate_adjacencies(row, col);
                        self.hmap.insert((row, col), new_adjacencies);
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
        fn calculate_adjacencies(&mut self, row: i64, col: i64) -> Vec<(i64, i64)> {
                // to avoid recasting we use delta of + of our fix
                let it_δ = (0..=2)
                        .cartesian_product(0..=2)
                        .map(|(x, y)| (x - 1, y - 1));
                let it_adj = it_δ.map(|(rδ, cδ)| (rδ + row, cδ + col));
                // // TODO: check bounds
                // // for right now this is merely a PERF issue
                // if r < 0 || c < 0  || r > r_max || c > c_max {

                it_adj.collect()
        }
}
