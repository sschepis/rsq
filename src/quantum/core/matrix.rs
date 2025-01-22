use std::ops::{Add, Mul};
use super::complex::Complex;

#[derive(Debug, Clone)]
pub struct ComplexMatrix {
    rows: usize,
    cols: usize,
    data: Vec<Complex>,
}

impl ComplexMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        ComplexMatrix {
            rows,
            cols,
            data: vec![Complex::new(0.0, 0.0); rows * cols],
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut matrix = Self::new(size, size);
        for i in 0..size {
            matrix.set(i, i, Complex::new(1.0, 0.0));
        }
        matrix
    }

    pub fn from_vector(vec: Vec<Complex>) -> Self {
        let n = vec.len();
        let mut matrix = Self::new(n, 1);
        matrix.data = vec;
        matrix
    }

    pub fn pauli_x() -> Self {
        let mut matrix = Self::new(2, 2);
        matrix.set(0, 1, Complex::new(1.0, 0.0));
        matrix.set(1, 0, Complex::new(1.0, 0.0));
        matrix
    }

    pub fn pauli_y() -> Self {
        let mut matrix = Self::new(2, 2);
        matrix.set(0, 1, Complex::new(0.0, -1.0));
        matrix.set(1, 0, Complex::new(0.0, 1.0));
        matrix
    }

    pub fn pauli_z() -> Self {
        let mut matrix = Self::new(2, 2);
        matrix.set(0, 0, Complex::new(1.0, 0.0));
        matrix.set(1, 1, Complex::new(-1.0, 0.0));
        matrix
    }

    pub fn get(&self, i: usize, j: usize) -> Complex {
        self.data[i * self.cols + j]
    }

    pub fn set(&mut self, i: usize, j: usize, value: Complex) {
        self.data[i * self.cols + j] = value;
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn add(&self, other: &ComplexMatrix) -> Result<ComplexMatrix, &'static str> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("Matrix dimensions must match for addition");
        }

        let mut result = ComplexMatrix::new(self.rows, self.cols);
        for i in 0..self.data.len() {
            result.data[i] = self.data[i] + other.data[i];
        }
        Ok(result)
    }

    pub fn multiply(&self, other: &ComplexMatrix) -> Result<ComplexMatrix, &'static str> {
        if self.cols != other.rows {
            return Err("Invalid matrix dimensions for multiplication");
        }

        let mut result = ComplexMatrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = Complex::new(0.0, 0.0);
                for k in 0..self.cols {
                    sum = sum + self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        Ok(result)
    }

    pub fn multiply_vector(&self, vec: &[Complex]) -> Result<Vec<Complex>, &'static str> {
        if self.cols != vec.len() {
            return Err("Matrix columns must match vector length");
        }

        let mut result = vec![Complex::new(0.0, 0.0); self.rows];
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i] = result[i] + self.get(i, j) * vec[j];
            }
        }
        Ok(result)
    }

    pub fn conjugate_transpose(&self) -> Self {
        let mut result = ComplexMatrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j).conjugate());
            }
        }
        result
    }

    pub fn trace(&self) -> Complex {
        assert_eq!(self.rows, self.cols);
        let mut sum = Complex::new(0.0, 0.0);
        for i in 0..self.rows {
            sum = sum + self.get(i, i);
        }
        sum
    }

    pub fn tensor_product(&self, other: &ComplexMatrix) -> Self {
        let new_rows = self.rows * other.rows;
        let new_cols = self.cols * other.cols;
        let mut result = ComplexMatrix::new(new_rows, new_cols);

        for i1 in 0..self.rows {
            for j1 in 0..self.cols {
                for i2 in 0..other.rows {
                    for j2 in 0..other.cols {
                        let prod = self.get(i1, j1) * other.get(i2, j2);
                        result.set(
                            i1 * other.rows + i2,
                            j1 * other.cols + j2,
                            prod
                        );
                    }
                }
            }
        }
        result
    }

    pub fn scale(&mut self, scalar: f64) {
        for i in 0..self.data.len() {
            self.data[i] = self.data[i] * Complex::new(scalar, 0.0);
        }
    }
}

impl Mul<Vec<Complex>> for ComplexMatrix {
    type Output = Vec<Complex>;

    fn mul(self, rhs: Vec<Complex>) -> Vec<Complex> {
        self.multiply_vector(&rhs).unwrap_or_else(|_| vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() {
        let mut a = ComplexMatrix::new(2, 2);
        let mut b = ComplexMatrix::new(2, 2);

        a.set(0, 0, Complex::new(1.0, 0.0));
        a.set(0, 1, Complex::new(2.0, 0.0));
        a.set(1, 0, Complex::new(3.0, 0.0));
        a.set(1, 1, Complex::new(4.0, 0.0));

        b.set(0, 0, Complex::new(1.0, 0.0));
        b.set(0, 1, Complex::new(0.0, 0.0));
        b.set(1, 0, Complex::new(0.0, 0.0));
        b.set(1, 1, Complex::new(1.0, 0.0));

        let c = a.multiply(&b).unwrap();

        assert_eq!(c.get(0, 0).real, 1.0);
        assert_eq!(c.get(0, 1).real, 2.0);
        assert_eq!(c.get(1, 0).real, 3.0);
        assert_eq!(c.get(1, 1).real, 4.0);
    }

    #[test]
    fn test_conjugate_transpose() {
        let mut a = ComplexMatrix::new(2, 2);
        a.set(0, 0, Complex::new(1.0, 1.0));
        a.set(0, 1, Complex::new(2.0, 2.0));
        a.set(1, 0, Complex::new(3.0, 3.0));
        a.set(1, 1, Complex::new(4.0, 4.0));

        let b = a.conjugate_transpose();

        assert_eq!(b.get(0, 0).imag, -1.0);
        assert_eq!(b.get(1, 0).imag, -2.0);
        assert_eq!(b.get(0, 1).imag, -3.0);
        assert_eq!(b.get(1, 1).imag, -4.0);
    }

    #[test]
    fn test_pauli_matrices() {
        let x = ComplexMatrix::pauli_x();
        let y = ComplexMatrix::pauli_y();
        let z = ComplexMatrix::pauli_z();

        assert_eq!(x.get(0, 1).real, 1.0);
        assert_eq!(x.get(1, 0).real, 1.0);

        assert_eq!(y.get(0, 1).imag, -1.0);
        assert_eq!(y.get(1, 0).imag, 1.0);

        assert_eq!(z.get(0, 0).real, 1.0);
        assert_eq!(z.get(1, 1).real, -1.0);
    }
}
