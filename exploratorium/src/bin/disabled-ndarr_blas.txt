use ndarray::Array2;
use ndarray_linalg::error::LinalgError;

fn create_matrix(rows: usize, cols: usize, one_positions: Vec<(usize, usize)>) -> Array2<f64> {
        let mut matrix = Array2::<f64>::zeros((rows, cols));
        for (row, col) in one_positions {
                matrix[(row, col)] = 1.0;
        }
        matrix
}

fn multiply_matrices(a: &Array2<f64>, b: &Array2<f64>) -> Result<Array2<f64>, LinalgError> {
        Ok(a.dot(b))
}

fn main() -> Result<(), LinalgError> {
        // Example: Create 3x3 matrices
        let matrix_a = create_matrix(3, 3, vec![(0, 1), (1, 2), (2, 0)]);
        let matrix_b = create_matrix(3, 3, vec![(0, 2), (1, 0), (2, 1)]);

        // Perform matrix multiplication
        let result = multiply_matrices(&matrix_a, &matrix_b)?;

        println!("Matrix A:\n{}", matrix_a);
        println!("Matrix B:\n{}", matrix_b);
        println!("Result of A * B:\n{}", result);

        Ok(())
}
