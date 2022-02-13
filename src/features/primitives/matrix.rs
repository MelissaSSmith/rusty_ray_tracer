use crate::features::primitives::operations::Operations;
use crate::features::primitives::tuple_trait::Tuple as TupleTrait;
use crate::features::primitives::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub struct Matrix {
    matrix: [[f64; 4]; 4]
}

impl Matrix{
    pub fn create(data: [[f64; 4]; 4]) -> Matrix {
        Matrix{
            matrix: data
        }
    }

    pub fn identity() -> Matrix {
        let identity = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ];
        Matrix::create(identity)
    }

    pub fn inverse(&self) -> Matrix {
        let mut inverse = [[0.0; 4]; 4];
        let d = self.determinant(4);
        for row in 0..4 {
            for col in 0..4 {
                inverse[col][row] = self.cofactor(row, col, 3) / d;
            }
        }

        Matrix::create(inverse)
    }

    pub fn get(&self, x: usize, y: usize) -> f64 {
        self.matrix[x][y]
    }

    pub(crate) fn set(&mut self, x: usize, y: usize, val: f64) {
        self.matrix[x][y] = val;
    }

    pub(crate) fn equals(&self, _matrix: Matrix) -> bool {
        let mut equals = true;

        for row in 0..4 {
            for column in 0..4 {
                let value = self.get(row, column);
                let other_value = _matrix.get(row, column);
                if value.equals(other_value) == false {
                    equals = false;
                    break;
                }
            }
        }

        equals
    }

    pub fn transpose(&self) -> Matrix {
        let columns = [
            [
                self.matrix[0][0],
                self.matrix[1][0],
                self.matrix[2][0],
                self.matrix[3][0],
            ],
            [
                self.matrix[0][1],
                self.matrix[1][1],
                self.matrix[2][1],
                self.matrix[3][1],
            ],
            [
                self.matrix[0][2],
                self.matrix[1][2],
                self.matrix[2][2],
                self.matrix[3][2],
            ],
            [
                self.matrix[0][3],
                self.matrix[1][3],
                self.matrix[2][3],
                self.matrix[3][3],
            ],
        ];
        Matrix::create(columns)
    }

    fn determinant(&self, size: usize) -> f64 {
        if size == 2 {
            let ad = self.get(0,0) * self.get(1,1);
            let bc = self.get(0,1) * self.get(1,0);
            return ad - bc;
        }
        let mut determinate = 0.0;
        for col in 0..4 {
            determinate += self.get(0, col) * self.cofactor(0, col, size - 1);
        }
        determinate
    }

    fn sub_matrix(&self, row: usize, column: usize) -> Matrix {
        let mut matrix = [[0.0; 4]; 4];
        for (nri, ri) in [0, 1, 2, 3].iter().filter(|&&x| x != row).enumerate() {
            for (nci, ci) in [0, 1, 2, 3].iter().filter(|&&x| x != column).enumerate() {
                matrix[nri][nci] = self.get(*ri, *ci);
            }
        }

        Matrix {matrix}
    }

    fn minor(&self, row: usize, column: usize, size: usize) -> f64{
        self.sub_matrix(row, column).determinant(size)
    }

    fn cofactor(&self, row: usize, column: usize, size: usize) -> f64 {
        let minor = self.minor(row, column, size);
        if (row + column) % 2 == 0 {
            return minor;
        }
        -minor
    }

    fn is_invertible(&self) -> bool {
        let determinant = self.determinant(4);
        determinant != 0.0
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut matrix = [[0.0; 4]; 4];
        let self_matrix = self.matrix;
        let other_matrix = rhs.matrix;

        for row in 0..4 {
            for col in 0..4 {
                matrix[row][col] = self_matrix[row][0] * other_matrix[0][col]
                    + self_matrix[row][1] * other_matrix[1][col]
                    + self_matrix[row][2] * other_matrix[2][col]
                    + self_matrix[row][3] * other_matrix[3][col];
            }
        }

        Matrix::create(matrix)
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl<T> std::ops::Mul<T> for Matrix where T: TupleTrait,
{
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        let x = self.get(0, 0) * rhs.x() + self.get(0, 1) * rhs.y()
            + self.get(0, 2) * rhs.z() + self.get(0, 3) * rhs.w();
        let y = self.get(1, 0) * rhs.x() + self.get(1, 1) * rhs.y()
            + self.get(1, 2) * rhs.z() + self.get(1, 3) * rhs.w();
        let z = self.get(2, 0) * rhs.x() + self.get(2, 1) * rhs.y()
            + self.get(2, 2) * rhs.z() + self.get(2, 3) * rhs.w();
        Self::Output::create(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use crate::features::primitives::matrix::Matrix;
    use crate::features::primitives::point::Point;
    use crate::features::primitives::tuple_trait::Tuple;

    #[test]
    fn test_create_matrix_4_4() {
        let vec_1 = [1.0, 2.0, 3.0, 4.0];
        let vec_2 = [5.5, 6.5, 7.5, 8.5];
        let vec_3 = [9.0, 10.0, 11.0, 12.0];
        let vec_4 = [13.5, 14.5, 15.5, 16.5];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

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
        let vec_1 = [-3.0, 5.0, 0.0, 0.0];
        let vec_2 = [1.0, -2.0, 0.0, 0.0];
        let vec_3 = [0.0, 0.0, 0.0, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 0.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert_eq!(-3.0, m.get(0,0));
        assert_eq!(5.0, m.get(0,1));
        assert_eq!(1.0, m.get(1,0));
        assert_eq!(-2.0, m.get(1,1));
    }

    #[test]
    fn test_create_matrix_3_3() {
        let vec_1 = [-3.0, 5.0, 0.0, 0.0];
        let vec_2 = [1.0, -2.0, -7.0, 0.0];
        let vec_3 = [0.0, 1.0, 1.0, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 0.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert_eq!(-3.0, m.get(0,0));
        assert_eq!(-2.0, m.get(1,1));
        assert_eq!(1.0, m.get(2,2));
    }

    #[test]
    fn test_identical_matrices_equal() {
        let vec_1 = [1.0, 2.0, 3.0, 4.0];
        let vec_2 = [5.5, 6.5, 7.5, 8.5];
        let vec_3 = [9.0, 10.0, 11.0, 12.0];
        let vec_4 = [13.5, 14.5, 15.5, 16.5];
        let m1 = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let vec_1 = [1.0, 2.0, 3.0, 4.0];
        let vec_2 = [5.5, 6.5, 7.5, 8.5];
        let vec_3 = [9.0, 10.0, 11.0, 12.0];
        let vec_4 = [13.5, 14.5, 15.5, 16.5];
        let m2 = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert!(m1.equals(m2));
    }

    #[test]
    fn test_different_matrices_not_equal() {
        let vec_1 = [1.0, 2.0, 3.0, 4.0];
        let vec_2 = [5.5, 6.5, 7.5, 8.5];
        let vec_3 = [9.0, 10.0, 11.0, 12.0];
        let vec_4 = [13.5, 14.5, 15.5, 16.5];
        let m1 = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let vec_1 = [2.0, 3.0, 4.0, 5.0];
        let vec_2 = [5.5, 6.5, 7.5, 8.5];
        let vec_3 = [9.0, 6.0, 11.0, 12.0];
        let vec_4 = [13.5, 14.5, 15.5, 16.5];
        let m2 = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert_eq!(false, m1.equals(m2));
    }

    #[test]
    fn test_multiply_matrices() {
        let vec_1 = [1.0, 2.0, 3.0, 4.0];
        let vec_2 = [5.0, 6.0, 7.0, 8.0];
        let vec_3 = [9.0, 8.0, 7.0, 6.0];
        let vec_4 = [5.0, 4.0, 3.0, 2.0];
        let m1 = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let vec_1 = [-2.0, 1.0, 2.0, 3.0];
        let vec_2 = [3.0, 2.0, 1.0, -1.0];
        let vec_3 = [4.0, 3.0, 6.0, 5.0];
        let vec_4 = [1.0, 2.0, 7.0, 8.0];
        let m2 = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let result = m1 * m2;

        let vec_1 = [20.0, 22.0, 50.0, 48.0];
        let vec_2 = [44.0, 54.0, 114.0, 108.0];
        let vec_3 = [40.0, 58.0, 110.0, 102.0];
        let vec_4 = [16.0, 26.0, 46.0, 42.0];
        let expected = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert!(expected.equals(result));
    }

    #[test]
    fn test_multiply_matrix_with_tuple() {
        let vec_1 = [1.0, 2.0, 3.0, 4.0];
        let vec_2 = [2.0, 4.0, 4.0, 2.0];
        let vec_3 = [8.0, 6.0, 4.0, 1.0];
        let vec_4 = [0.0, 0.0, 0.0, 1.0];
        let m1 = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let tuple = Point::create(1.0, 2.0, 3.0);

        let result = m1 * tuple;

        let expected = Point::create(18.0, 24.0, 33.0);

        assert_eq!(expected.x(), result.x());
        assert_eq!(expected.y(), result.y());
        assert_eq!(expected.z(), result.z());
        assert_eq!(expected.w(), result.w());
    }

    #[test]
    fn test_multiply_by_identity_matrix() {
        let vec_1 = [1.0, 2.0, 3.0, 4.0];
        let vec_2 = [5.0, 6.0, 7.0, 8.0];
        let vec_3 = [9.0, 8.0, 7.0, 6.0];
        let vec_4 = [5.0, 4.0, 3.0, 2.0];
        let m1 = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let vec_1 = [1.0, 0.0, 0.0, 0.0];
        let vec_2 = [0.0, 1.0, 0.0, 0.0];
        let vec_3 = [0.0, 0.0, 1.0, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 1.0];
        let m2 = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let result = m1 * m2;

        assert!(m1.equals(result));
    }

    #[test]
    fn test_multiply_tuple_by_identity_matrix() {
        let vec_1 = [1.0, 0.0, 0.0, 0.0];
        let vec_2 = [0.0, 1.0, 0.0, 0.0];
        let vec_3 = [0.0, 0.0, 1.0, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 1.0];
        let m2 = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let tuple = Point::create(1.0, 2.0, 3.0);

        let result = m2 * tuple;

        let expected = Point::create(1.0, 2.0, 3.0);
        assert_eq!(expected.x(), result.x());
        assert_eq!(expected.y(), result.y());
        assert_eq!(expected.z(), result.z());
        assert_eq!(expected.w(), result.w());
    }

    #[test]
    fn test_transpose() {
        let vec_1 = [0.0, 9.0, 3.0, 0.0];
        let vec_2 = [9.0, 8.0, 0.0, 8.0];
        let vec_3 = [1.0, 8.0, 5.0, 3.0];
        let vec_4 = [0.0, 0.0, 5.0, 8.0];
        let m1 = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let result = m1.transpose();

        let vec_1 = [0.0, 9.0, 1.0, 0.0];
        let vec_2 = [9.0, 8.0, 8.0, 0.0];
        let vec_3 = [3.0, 0.0, 5.0, 5.0];
        let vec_4 = [0.0, 8.0, 3.0, 8.0];
        let expected = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert!(expected.equals(result));
    }

    #[test]
    fn test_transpose_identity_matrix() {
        let vec_1 = [1.0, 0.0, 0.0, 0.0];
        let vec_2 = [0.0, 1.0, 0.0, 0.0];
        let vec_3 = [0.0, 0.0, 1.0, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 1.0];
        let m2 = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let result = m2.transpose();

        assert!(m2.equals(result));
    }

    #[test]
    fn test_determinate_of_2_2_matrix() {
        let vec_1 = [1.0, 5.0, 0.0, 0.0];
        let vec_2 = [-3.0, 2.0, 0.0, 0.0];
        let vec_3 = [0.0, 0.0, 0.0, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 0.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let determinant = m.determinant(2);

        assert_eq!(17.0, determinant);
    }

    #[test]
    fn test_sub_matrix_of_3_3_matrix_is_2_2_matrix() {
        let vec_1 = [1.0, 5.0, 0.0, 0.0];
        let vec_2 = [-3.0, 2.0, 7.0, 0.0];
        let vec_3 = [0.0, 6.0, -3.0, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 0.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let result = m.sub_matrix(0, 2);

        let vec_1 = [-3.0, 2.0, 0.0, 0.0];
        let vec_2 = [0.0, 6.0, 0.0, 0.0];
        let vec_3 = [0.0, 0.0, 0.0, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 0.0];
        let expected = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert!(result.equals(expected));
    }

    #[test]
    fn test_sub_matrix_of_4_4_matrix_is_3_3_matrix() {
        let vec_1 = [-6.0, 1.0, 1.0, 6.0];
        let vec_2 = [-8.0, 5.0, 8.0, 6.0];
        let vec_3 = [-1.0, 0.0, 8.0, 2.0];
        let vec_4 = [-7.0, 1.0, -1.0, 1.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let result = m.sub_matrix(2, 1);

        let vec_1 = [-6.0, 1.0, 6.0, 0.0];
        let vec_2 = [-8.0, 8.0, 6.0, 0.0];
        let vec_3 = [-7.0, -1.0, 1.0, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 0.0];
        let expected = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert!(result.equals(expected));
    }

    #[test]
    fn test_calculate_minor() {
        let vec_1 = [3.0, 5.0, 0.0, 0.0];
        let vec_2 = [2.0, -1.0, -7.0, 0.0];
        let vec_3 = [6.0, -1.0, 5.0, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 0.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let sub_matrix = m.sub_matrix(1, 0);

        let sub_matrix_determinant = sub_matrix.determinant(3);
        let minor = m.minor(1, 0, 3);

        assert_eq!(sub_matrix_determinant, minor);
    }

    #[test]
    fn test_calculate_cofactor() {
        let vec_1 = [3.0, 5.0, 0.0, 0.0];
        let vec_2 = [2.0, -1.0, -7.0, 0.0];
        let vec_3 = [6.0, -1.0, 5.0, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 0.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let minor_a = m.minor(0, 0, 2);
        let cofactor_a = m.cofactor(0, 0, 2);
        let minor_b = m.minor(1, 0, 2);
        let cofactor_b = m.cofactor(1, 0, 2);

        assert_eq!(minor_a, -12.0);
        assert_eq!(cofactor_a, -12.0);
        assert_eq!(minor_b, 25.0);
        assert_eq!(cofactor_b, -25.0);
    }

    #[test]
    fn test_calculate_determinant_of_3_3_matrix() {
        let vec_1 = [1.0, 2.0, 6.0, 0.0];
        let vec_2 = [-5.0, 8.0, -4.0, 0.0];
        let vec_3 = [2.0, 6.0, 4.0, 0.0];
        let vec_4 = [0.0, 0.0, 0.0, 0.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let cofactor_a = m.cofactor(0, 0, 2);
        let cofactor_b = m.cofactor(0, 1, 2);
        let cofactor_c = m.cofactor(0, 2, 2);
        let determinant = m.determinant(3);

        assert_eq!(cofactor_a, 56.0);
        assert_eq!(cofactor_b, 12.0);
        assert_eq!(cofactor_c, -46.0);
        assert_eq!(determinant, -196.0);
    }

    #[test]
    fn test_calculate_determinant_of_4_4_matrix() {
        let vec_1 = [-2.0, -8.0, 3.0, 5.0];
        let vec_2 = [-3.0, 1.0, 7.0, 3.0];
        let vec_3 = [1.0, 2.0, -9.0, 6.0];
        let vec_4 = [-6.0, 7.0, 7.0, -9.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let cofactor_a = m.cofactor(0, 0, 3);
        let cofactor_b = m.cofactor(0, 1, 3);
        let cofactor_c = m.cofactor(0, 2, 3);
        let cofactor_d = m.cofactor(0, 3, 3);
        let determinant = m.determinant(4);

        assert_eq!(cofactor_a, 690.0);
        assert_eq!(cofactor_b, 447.0);
        assert_eq!(cofactor_c, 210.0);
        assert_eq!(cofactor_d, 51.0);
        assert_eq!(determinant, -4071.0);
    }

    #[test]
    fn test_matrix_is_invertible() {
        let vec_1 = [6.0, 4.0, 4.0, 4.0];
        let vec_2 = [5.0, 5.0, 7.0, 6.0];
        let vec_3 = [4.0, -9.0, 3.0, -7.0];
        let vec_4 = [9.0, 1.0, 7.0, -6.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert!(m.is_invertible());
    }

    #[test]
    fn test_matrix_is_not_invertible() {
        let vec_1 = [-4.0, 2.0, -2.0, -3.0];
        let vec_2 = [9.0, 6.0, 2.0, 6.0];
        let vec_3 = [0.0, -5.0, 1.0, -5.0];
        let vec_4 = [0.0, 0.0, 0.0, 0.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert_eq!(false, m.is_invertible());
    }

    #[test]
    fn test_inverse_matrix() {
        let vec_1 = [-5.0, 2.0, 6.0, -8.0];
        let vec_2 = [1.0, -5.0, 1.0, 8.0];
        let vec_3 = [7.0, 7.0, -6.0, -7.0];
        let vec_4 = [1.0, -3.0, 7.0, 4.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let inverse_m = m.inverse();

        assert_eq!(532.0, m.determinant(4));
        assert_eq!(-160.0, m.cofactor(2, 3, 3));
        assert_eq!(-160.0/532.0, inverse_m.get(3, 2));
        assert_eq!(105.0, m.cofactor(3, 2, 3));
        assert_eq!(105.0/532.0, inverse_m.get(2, 3));

        let vec_1 = [0.21805, 0.45113, 0.24060, -0.04511];
        let vec_2 = [-0.80827, -1.45677, -0.44361, 0.52068];
        let vec_3 = [-0.07895, -0.22368, -0.05263, 0.19737];
        let vec_4 = [-0.52256, -0.81391, -0.30075, 0.30639];
        let expected = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert!(expected.equals(inverse_m));
    }

    #[test]
    fn test_inverse_matrix_2() {
        let vec_1 = [8.0, -5.0, 9.0, 2.0];
        let vec_2 = [7.0, 5.0, 6.0, 1.0];
        let vec_3 = [-6.0, 0.0, 9.0, 6.0];
        let vec_4 = [-3.0, 0.0, -9.0, -4.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let inverse_m = m.inverse();

        let vec_1 = [-0.15385, -0.15385, -0.28205, -0.53846];
        let vec_2 = [-0.07692, 0.12308, 0.02564, 0.03077];
        let vec_3 = [0.35897, 0.35897, 0.43590, 0.92308];
        let vec_4 = [-0.69231, -0.69231, -0.76923, -1.92308];
        let expected = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert!(expected.equals(inverse_m));
    }

    #[test]
    fn test_inverse_matrix_3() {
        let vec_1 = [9.0, 3.0, 0.0, 9.0];
        let vec_2 = [-5.0, -2.0, -6.0, -3.0];
        let vec_3 = [-4.0, 9.0, 6.0, 4.0];
        let vec_4 = [-7.0, 6.0, 6.0, 2.0];
        let m = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let inverse_m = m.inverse();

        let vec_1 = [-0.04074, -0.07778, 0.14444, -0.22222];
        let vec_2 = [-0.07778, 0.03333, 0.36667, -0.33333];
        let vec_3 = [-0.02901, -0.14630, -0.10926, 0.12963];
        let vec_4 = [0.17778, 0.06667, -0.26667, 0.33333];
        let expected = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        assert!(expected.equals(inverse_m));
    }

    #[test]
    fn test_multiple_product_by_its_inverse() {
        let vec_1 = [3.0, -9.0, 7.0, 3.0];
        let vec_2 = [3.0, -8.0, 2.0, -9.0];
        let vec_3 = [-4.0, 4.0, 4.0, 1.0];
        let vec_4 = [-6.0, 5.0, 1.0, 1.0];
        let a = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let vec_1 = [8.0, 2.0, 2.0, 2.0];
        let vec_2 = [3.0, -1.0, 7.0, 0.0];
        let vec_3 = [7.0, 0.0, 5.0, 4.0];
        let vec_4 = [6.0, -2.0, 0.0, 5.0];
        let b = Matrix::create([vec_1, vec_2, vec_3, vec_4]);

        let c = a * b;

        let product = c * b.inverse();

        assert!(a.equals(product));
    }
}