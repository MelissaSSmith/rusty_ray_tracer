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
}