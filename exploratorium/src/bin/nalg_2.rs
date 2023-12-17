use nalgebra::DMatrix;

fn create_matrix(rows: usize, cols: usize, one_positions: Vec<(usize, usize)>) -> DMatrix<i32> {
        let mut matrix = DMatrix::from_element(rows, cols, 0);
        for (row, col) in one_positions {
                matrix[(row, col)] = 1;
        }
        matrix
}

fn multiply_matrices(a: DMatrix<i32>, b: DMatrix<i32>) -> DMatrix<i32> {
        a * b
}

fn main() {
        let matrix_a = create_matrix(3, 3, vec![(0, 1), (1, 2), (2, 0)]);
        let matrix_b = create_matrix(3, 3, vec![(0, 2), (1, 0), (2, 1)]);

        let result = multiply_matrices(matrix_a, matrix_b);
        println!("Result:\n{}", result);
}
