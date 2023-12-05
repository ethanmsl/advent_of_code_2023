//! Objects for Day-03
use derive_more::{AsMut, AsRef, Constructor, Display, IntoIterator};
use once_cell::sync::Lazy;
use regex::Regex;

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
        pub fn register_number(&mut self, row: usize, raw_line: &str) {
                static re_number: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

                re_number.find_iter(raw_line).for_each(|m| {
                        let (h_start, h_end) = (m.start(), m.end());
                        let len = m.len();
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
        start_loc: (usize, usize),
        len: usize,
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
        pub fn id(&self) -> (usize, usize) {
                self.start_loc
        }
}
