//! # Library to define range arithmetic.
//!
//! ## e.g. <conceptual>
//!
//! (0..5).collapse_(2..6) = [(0..6)]
//! (0..5).split____(2..6) = [(0..2), (2..5), (5..6)]
//! (0..5).intersect(2..6) = [(2..5)]
//! (0..5).left_take(2..6) = [(0..5), (5..6)]
//!
//! (0..5).collapse_(5..6) = [(0..5), (5..6)]
//! (0..5).split____(5..6) = [(0..5), (5..6)]
//! (0..5).intersect(5..6) = []
//! (0..5).left_take(5..6) = [(0..5), (5..6)]
//!
//! ^ possibly try variants, but seems less performant ... maybe more if not allocating
//!   ... maybe it could return a tuple or array with Some/None...
//!
//! # also <conceptual>
//! give it an extensions that does the offset addition for the ranges
//! ^ maybe that should be in main library...
//!
//! ## implementation
//! extend Range object ... (?)
//!
//! ## Next Step
//! I want to use the range methods to create RangeBump methods
//! - Splitting and adding bumps of both if any
//! - Collapsing conditionally on identical offset
//!
//! # Alternative:
//! Skip the ranges entirely and just model the boundaries of i64, with offset in each
//! Need a special `None`` value for adding seeds so we only get 'populated' ranges
//! Actually ... that might be simplest
