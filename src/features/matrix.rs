use array2d::Array2D;
use crate::features::tuple::Tuple;

struct Matrix {
    matrix: Array2D<f64>
}

impl Matrix{
    pub fn create(vecs: Vec<Vec<f64>>) -> Matrix {
        Matrix{
            matrix: Array2D::from_rows(&vecs)
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f64 {
        let item = self.matrix.get(x, y);
        match item {
            None => {-1.0}
            Some(m) => {*m}
        }
    }

    pub fn set(&mut self, x: usize, y: usize, val: f64) {
        self.matrix.set(x, y, val);
    }

    pub fn equals(&self, _matrix: Matrix) -> bool {
        self.matrix.eq(&_matrix.matrix)
    }

    pub fn multiply(&self, _matrix: Matrix) -> Matrix {
        let mut vecs: Vec<Vec<f64>> = Vec::with_capacity(self.matrix.row_len());
        for _ in 0..self.matrix.row_len() {
            vecs.push(vec![0.0; self.matrix.row_len()]);
        }
        let mut new_matrix = Matrix::create(vecs);
        let rows = self.matrix.as_rows();
        let columns = _matrix.matrix.as_columns();
        for row in 0..rows.len() {
            let row_tuple = Tuple::convert_to_tuple(rows.get(row).unwrap());
            for column in 0..columns.len() {
                let column_tuple = Tuple::convert_to_tuple(columns.get(column).unwrap());
                let new_value = row_tuple.dot(column_tuple);
                new_matrix.set(row, column, new_value);
            }
        }

        new_matrix
    }

    pub fn multiply_by_tuple(&self, _tuple: Tuple) -> Tuple {
        let rows = self.matrix.as_rows();
        let mut result_list = vec![];
        for row in 0..rows.len() {
            let row_tuple = Tuple::convert_to_tuple(rows.get(row).unwrap());
            result_list.push(_tuple.dot(row_tuple));
        }

        Tuple::convert_to_tuple(&result_list)
    }

    pub fn transpose(&self) -> Matrix {
        let columns = self.matrix.as_columns();
        Matrix::create(columns)
    }

    fn determinant(&self) -> f64 {
        if self.matrix.row_len() == 2 && self.matrix.column_len() == 2 {
            let ad = self.get(0,0) * self.get(1,1);
            let bc = self.get(0,1) * self.get(1,0);
            return ad - bc;
        }
        let mut determinate = 0.0;
        for col in 0..self.matrix.column_len() {
            determinate = determinate + self.get(0, col) * self.cofactor(0, col);
        }
        determinate
    }

    fn sub_matrix(&self, row: usize, column: usize) -> Matrix {
        let sub_matrix_size = self.matrix.row_len() - 1;
        let mut vecs: Vec<Vec<f64>> = Vec::with_capacity(sub_matrix_size);
        for r in 0..self.matrix.as_rows().len() {
            if r != row {
                let mut row_vec = vec![];
                for c in 0..self.matrix.as_columns().len() {
                    if c != column {
                        row_vec.push(self.get(r, c));
                    }
                }
                vecs.push(row_vec);
            }
        }

        Matrix::create(vecs)
    }

    fn minor(&self, row: usize, column: usize) -> f64{
        let sub_matrix = self.sub_matrix(row, column);
        sub_matrix.determinant()
    }

    fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            return minor;
        }
        -minor
    }
}

#[cfg(test)]
mod tests {
    use crate::features::matrix::Matrix;
    use crate::features::tuple::Tuple;

    #[test]
    fn test_create_matrix_4_4() {
        let vec_1 = vec![1.0, 2.0, 3.0, 4.0];
        let vec_2 = vec![5.5, 6.5, 7.5, 8.5];
        let vec_3 = vec![9.0, 10.0, 11.0, 12.0];
        let vec_4 = vec![13.5, 14.5, 15.5, 16.5];
        let m = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        assert_eq!(1.0, m.get(0,0));
        assert_eq!(4.0, m.get(0,3));
        assert_eq!(5.5, m.get(1,0));
        assert_eq!(7.5, m.get(1,2));
        assert_eq!(11.0, m.get(2,2));
        assert_eq!(13.5, m.get(3,0));
        assert_eq!(15.5, m.get(3,2));
    }

    #[test]
    fn test_create_matrix_2_2() {
        let vec_1 = vec![-3.0, 5.0];
        let vec_2 = vec![1.0, -2.0];
        let m = Matrix::create(vec![vec_1, vec_2]);

        assert_eq!(-3.0, m.get(0,0));
        assert_eq!(5.0, m.get(0,1));
        assert_eq!(1.0, m.get(1,0));
        assert_eq!(-2.0, m.get(1,1));
    }

    #[test]
    fn test_create_matrix_3_3() {
        let vec_1 = vec![-3.0, 5.0, 0.0];
        let vec_2 = vec![1.0, -2.0, -7.0];
        let vec_3 = vec![0.0, 1.0, 1.0];
        let m = Matrix::create(vec![vec_1, vec_2, vec_3]);

        assert_eq!(-3.0, m.get(0,0));
        assert_eq!(-2.0, m.get(1,1));
        assert_eq!(1.0, m.get(2,2));
    }

    #[test]
    fn test_identical_matrices_equal() {
        let vec_1 = vec![1.0, 2.0, 3.0, 4.0];
        let vec_2 = vec![5.5, 6.5, 7.5, 8.5];
        let vec_3 = vec![9.0, 10.0, 11.0, 12.0];
        let vec_4 = vec![13.5, 14.5, 15.5, 16.5];
        let m1 = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        let vec_1 = vec![1.0, 2.0, 3.0, 4.0];
        let vec_2 = vec![5.5, 6.5, 7.5, 8.5];
        let vec_3 = vec![9.0, 10.0, 11.0, 12.0];
        let vec_4 = vec![13.5, 14.5, 15.5, 16.5];
        let m2 = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        assert!(m1.equals(m2));
    }

    #[test]
    fn test_different_matrices_not_equal() {
        let vec_1 = vec![1.0, 2.0, 3.0, 4.0];
        let vec_2 = vec![5.5, 6.5, 7.5, 8.5];
        let vec_3 = vec![9.0, 10.0, 11.0, 12.0];
        let vec_4 = vec![13.5, 14.5, 15.5, 16.5];
        let m1 = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        let vec_1 = vec![2.0, 3.0, 4.0, 5.0];
        let vec_2 = vec![5.5, 6.5, 7.5, 8.5];
        let vec_3 = vec![9.0, 10.0, 11.0, 12.0];
        let vec_4 = vec![13.5, 14.5, 15.5, 16.5];
        let m2 = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        assert_eq!(false, m1.equals(m2));
    }

    #[test]
    fn test_multiply_matrices() {
        let vec_1 = vec![1.0, 2.0, 3.0, 4.0];
        let vec_2 = vec![5.0, 6.0, 7.0, 8.0];
        let vec_3 = vec![9.0, 8.0, 7.0, 6.0];
        let vec_4 = vec![5.0, 4.0, 3.0, 2.0];
        let m1 = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        let vec_1 = vec![-2.0, 1.0, 2.0, 3.0];
        let vec_2 = vec![3.0, 2.0, 1.0, -1.0];
        let vec_3 = vec![4.0, 3.0, 6.0, 5.0];
        let vec_4 = vec![1.0, 2.0, 7.0, 8.0];
        let m2 = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        let result = m1.multiply(m2);

        let vec_1 = vec![20.0, 22.0, 50.0, 48.0];
        let vec_2 = vec![44.0, 54.0, 114.0, 108.0];
        let vec_3 = vec![40.0, 58.0, 110.0, 102.0];
        let vec_4 = vec![16.0, 26.0, 46.0, 42.0];
        let expected = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        assert!(expected.equals(result));
    }

    #[test]
    fn test_multiply_matrix_with_tuple() {
        let vec_1 = vec![1.0, 2.0, 3.0, 4.0];
        let vec_2 = vec![2.0, 4.0, 4.0, 2.0];
        let vec_3 = vec![8.0, 6.0, 4.0, 1.0];
        let vec_4 = vec![0.0, 0.0, 0.0, 1.0];
        let m1 = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        let tuple = Tuple::create(1.0, 2.0, 3.0, 1.0);

        let result = m1.multiply_by_tuple(tuple);

        let expected = Tuple::create(18.0, 24.0, 33.0, 1.0);

        assert_eq!(expected.x, result.x);
        assert_eq!(expected.y, result.y);
        assert_eq!(expected.z, result.z);
        assert_eq!(expected.w, result.w);
    }

    #[test]
    fn test_multiply_by_identity_matrix() {
        let vec_1 = vec![1.0, 2.0, 3.0, 4.0];
        let vec_2 = vec![5.0, 6.0, 7.0, 8.0];
        let vec_3 = vec![9.0, 8.0, 7.0, 6.0];
        let vec_4 = vec![5.0, 4.0, 3.0, 2.0];
        let m1 = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        let vec_1 = vec![1.0, 0.0, 0.0, 0.0];
        let vec_2 = vec![0.0, 1.0, 0.0, 0.0];
        let vec_3 = vec![0.0, 0.0, 1.0, 0.0];
        let vec_4 = vec![0.0, 0.0, 0.0, 1.0];
        let m2 = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        let result = m1.multiply(m2);

        assert!(m1.equals(result));
    }

    #[test]
    fn test_multiply_tuple_by_identity_matrix() {
        let vec_1 = vec![1.0, 0.0, 0.0, 0.0];
        let vec_2 = vec![0.0, 1.0, 0.0, 0.0];
        let vec_3 = vec![0.0, 0.0, 1.0, 0.0];
        let vec_4 = vec![0.0, 0.0, 0.0, 1.0];
        let m2 = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        let tuple = Tuple::create(1.0, 2.0, 3.0, 1.0);

        let result = m2.multiply_by_tuple(tuple);

        let expected = Tuple::create(1.0, 2.0, 3.0, 1.0);
        assert_eq!(expected.x, result.x);
        assert_eq!(expected.y, result.y);
        assert_eq!(expected.z, result.z);
        assert_eq!(expected.w, result.w);
    }

    #[test]
    fn test_transpose() {
        let vec_1 = vec![0.0, 9.0, 3.0, 0.0];
        let vec_2 = vec![9.0, 8.0, 0.0, 8.0];
        let vec_3 = vec![1.0, 8.0, 5.0, 3.0];
        let vec_4 = vec![0.0, 0.0, 5.0, 8.0];
        let m1 = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        let result = m1.transpose();

        let vec_1 = vec![0.0, 9.0, 1.0, 0.0];
        let vec_2 = vec![9.0, 8.0, 8.0, 0.0];
        let vec_3 = vec![3.0, 0.0, 5.0, 5.0];
        let vec_4 = vec![0.0, 8.0, 3.0, 8.0];
        let expected = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        assert!(expected.equals(result));
    }

    #[test]
    fn test_transpose_identity_matrix() {
        let vec_1 = vec![1.0, 0.0, 0.0, 0.0];
        let vec_2 = vec![0.0, 1.0, 0.0, 0.0];
        let vec_3 = vec![0.0, 0.0, 1.0, 0.0];
        let vec_4 = vec![0.0, 0.0, 0.0, 1.0];
        let m2 = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        let result = m2.transpose();

        assert!(m2.equals(result));
    }

    #[test]
    fn test_determinate_of_2_2_matrix() {
        let vec_1 = vec![1.0, 5.0];
        let vec_2 = vec![-3.0, 2.0];
        let m = Matrix::create(vec![vec_1, vec_2]);

        let determinant = m.determinant();

        assert_eq!(17.0, determinant);
    }

    #[test]
    fn test_sub_matrix_of_3_3_matrix_is_2_2_matrix() {
        let vec_1 = vec![1.0, 5.0, 0.0];
        let vec_2 = vec![-3.0, 2.0, 7.0];
        let vec_3 = vec![0.0, 6.0, -3.0];
        let m = Matrix::create(vec![vec_1, vec_2, vec_3]);

        let result = m.sub_matrix(0, 2);

        let vec_1 = vec![-3.0, 2.0];
        let vec_2 = vec![0.0, 6.0];
        let expected = Matrix::create(vec![vec_1, vec_2]);

        assert!(result.equals(expected));
    }

    #[test]
    fn test_sub_matrix_of_4_4_matrix_is_3_3_matrix() {
        let vec_1 = vec![-6.0, 1.0, 1.0, 6.0];
        let vec_2 = vec![-8.0, 5.0, 8.0, 6.0];
        let vec_3 = vec![-1.0, 0.0, 8.0, 2.0];
        let vec_4 = vec![-7.0, 1.0, -1.0, 1.0];
        let m = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        let result = m.sub_matrix(2, 1);

        let vec_1 = vec![-6.0, 1.0, 6.0];
        let vec_2 = vec![-8.0, 8.0, 6.0];
        let vec_3 = vec![-7.0, -1.0, 1.0];
        let expected = Matrix::create(vec![vec_1, vec_2, vec_3]);

        assert!(result.equals(expected));
    }

    #[test]
    fn test_calculate_minor() {
        let vec_1 = vec![3.0, 5.0, 0.0];
        let vec_2 = vec![2.0, -1.0, -7.0];
        let vec_3 = vec![6.0, -1.0, 5.0];
        let m = Matrix::create(vec![vec_1, vec_2, vec_3]);

        let sub_matrix = m.sub_matrix(1, 0);

        let sub_matrix_determinant = sub_matrix.determinant();
        let minor = m.minor(1, 0);

        assert_eq!(sub_matrix_determinant, minor);
    }

    #[test]
    fn test_calculate_cofactor() {
        let vec_1 = vec![3.0, 5.0, 0.0];
        let vec_2 = vec![2.0, -1.0, -7.0];
        let vec_3 = vec![6.0, -1.0, 5.0];
        let m = Matrix::create(vec![vec_1, vec_2, vec_3]);

        let minor_a = m.minor(0, 0);
        let cofactor_a = m.cofactor(0, 0);
        let minor_b = m.minor(1, 0);
        let cofactor_b = m.cofactor(1, 0);

        assert_eq!(minor_a, -12.0);
        assert_eq!(cofactor_a, -12.0);
        assert_eq!(minor_b, 25.0);
        assert_eq!(cofactor_b, -25.0);
    }

    #[test]
    fn test_calculate_determinant_of_3_3_matrix() {
        let vec_1 = vec![1.0, 2.0, 6.0];
        let vec_2 = vec![-5.0, 8.0, -4.0];
        let vec_3 = vec![2.0, 6.0, 4.0];
        let m = Matrix::create(vec![vec_1, vec_2, vec_3]);

        let cofactor_a = m.cofactor(0, 0);
        let cofactor_b = m.cofactor(0, 1);
        let cofactor_c = m.cofactor(0, 2);
        let determinant = m.determinant();

        assert_eq!(cofactor_a, 56.0);
        assert_eq!(cofactor_b, 12.0);
        assert_eq!(cofactor_c, -46.0);
        assert_eq!(determinant, -196.0);
    }

    #[test]
    fn test_calculate_determinant_of_4_4_matrix() {
        let vec_1 = vec![-2.0, -8.0, 3.0, 5.0];
        let vec_2 = vec![-3.0, 1.0, 7.0, 3.0];
        let vec_3 = vec![1.0, 2.0, -9.0, 6.0];
        let vec_4 = vec![-6.0, 7.0, 7.0, -9.0];
        let m = Matrix::create(vec![vec_1, vec_2, vec_3, vec_4]);

        let cofactor_a = m.cofactor(0, 0);
        let cofactor_b = m.cofactor(0, 1);
        let cofactor_c = m.cofactor(0, 2);
        let cofactor_d = m.cofactor(0, 3);
        let determinant = m.determinant();

        assert_eq!(cofactor_a, 690.0);
        assert_eq!(cofactor_b, 447.0);
        assert_eq!(cofactor_c, 210.0);
        assert_eq!(cofactor_d, 51.0);
        assert_eq!(determinant, -4071.0);
    }
}