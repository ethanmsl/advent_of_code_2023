//! NAlgebra examples

use anyhow::Result;
use nalgebra::{Matrix2, Matrix3, Matrix4, Matrix4x6, Vector2, Vector3, Vector4};
use tracing::event;

#[rustfmt::skip]

#[tracing::instrument]
fn main() -> Result<()> {
        tracing_subscriber::fmt::init();

        let v2 = Vector2::new(1.0, 2.0);
        let v3 = Vector3::new(1.0, 2.0, 3.0);
        let v4 = Vector4::new(1.0, 2.0, 3.0, 4.0);

        let m2 = Matrix2::new(1.0, 2.0,
                              3.0, 4.0);

        let m3 = Matrix3::new(1.0, 2.0, 0.0,
                              4.0, 5.0, 0.0,
                              0.0, 0.0, 0.0);
        let m4 = Matrix4::new(0.1, 2.0, 3.0, 4.0,
                              0.5, 6.0, 7.0, 8.0,
                              0.9, 0.0, 1.0, 2.0, 
                              0.3, 4.0, 5.0, 6.0,);
        let m4by6 = Matrix4x6::new(
                                    1.0, 2.0, 3.0, 4.0, 5.0, 6.0,
                                    7.0, 8.0, 9.0, 0.0, 1.0, 2.0,
                                    3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
                                    9.0, 0.0, 1.0, 2.0, 3.0, 4.0,
        );

        let v2_2 = m2 * v2;
        let v3_2 = m3 * v3;
        let v4_2 = m4 * v4;

        // do some math
        println!("v2_2 = {}", v2_2);
        println!("v3_2 = {}", v3_2);
        println!("v4_2 = {}", v4_2);

        event!(tracing::Level::INFO, "m2 = {}", m2);
        event!(tracing::Level::INFO, "m3 = {}", m3);
        event!(tracing::Level::INFO, "m4 = {}", m4);
        event!(tracing::Level::INFO, "m4by6 = {}", m4by6);

        // 4x4 matrix to the 10th power
        let m4_10 = m4.pow(10);
        event!(tracing::Level::INFO, "m4_10 = {}", m4_10);

        // 4x4 matrix to the 10th power
        let m3_3 = m3.pow(2);
        event!(tracing::Level::INFO, "m3_3 = {}", m3_3);

        // making a bool matrix
        let m3_bool = m3.map(|x| x > 0.0);
        event!(tracing::Level::INFO, "m3_bool = {}", m3_bool);
        // event!(tracing::Level::INFO, "m3_bool.pow(2) = {}", m3_bool.pow(2));

        Ok(())
}
