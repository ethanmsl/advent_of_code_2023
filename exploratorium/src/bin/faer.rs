//! Trying faer LA crate.
#![allow(warnings)]

use anyhow::Result;
use faer::{mat, prelude::*, Mat};
use tracing::event;

fn main() -> Result<()> {
        tracing_subscriber::fmt::init();

        // empty 0x0 matrix
        let m0: Mat<f64> = Mat::new();

        // zeroed 4x3 matrix
        let m1: Mat<f64> = Mat::zeros(4, 3);

        // 3x3 identity matrix
        let m2 = Mat::from_fn(3, 3, |i, j| if i == j { 1.0 } else { 0.0 });

        // 4x2 matrix with custom data
        let m3 = mat![[4.93, 2.41], [5.43, 4.33], [9.83, 1.59], [7.13, 5.02_f64],];

        // compute the qr decomposition of a matrix
        let qr_decomposition = m3.qr();

        event!(tracing::Level::INFO, "m3 = {:?}", m3);
        Ok(())
}
