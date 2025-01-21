use wasm_bindgen::prelude::*;
use super::complex::Complex;
use std::ops::Mul;

#[wasm_bindgen]
#[derive(Clone)]
#[derive(Debug)]
pub struct ComplexMatrix {
    rows: usize,
    cols: usize,
    data: Vec<Complex>,
}

#[wasm_bindgen]
impl ComplexMatrix {
    #[wasm_bindgen(constructor)]
    pub fn new(rows: usize, cols: usize) -> ComplexMatrix {
        let data = vec![Complex::new(0.0, 0.0); rows * cols];
        ComplexMatrix { rows, cols, data }
    }

    pub fn identity(size: usize) -> ComplexMatrix {
        let mut matrix = ComplexMatrix::new(size, size);
        for i in 0..size {
            matrix.set(i, i, &Complex::new(1.0, 0.0));
        }
        matrix
    }

    pub fn pauli_x() -> ComplexMatrix {
        let mut matrix = ComplexMatrix::new(2, 2);
        matrix.set(0, 1, &Complex::new(1.0, 0.0));
        matrix.set(1, 0, &Complex::new(1.0, 0.0));
        matrix
    }

    pub fn pauli_y() -> ComplexMatrix {
        let mut matrix = ComplexMatrix::new(2, 2);
        matrix.set(0, 1, &Complex::new(0.0, -1.0));
        matrix.set(1, 0, &Complex::new(0.0, 1.0));
        matrix
    }

    pub fn pauli_z() -> ComplexMatrix {
        let mut matrix = ComplexMatrix::new(2, 2);
        matrix.set(0, 0, &Complex::new(1.0, 0.0));
        matrix.set(1, 1, &Complex::new(-1.0, 0.0));
        matrix
    }

    pub fn get(&self, row: usize, col: usize) -> Complex {
        self.data[row * self.cols + col].clone()
    }

    pub fn set(&mut self, row: usize, col: usize, value: &Complex) {
        self.data[row * self.cols + col] = value.clone();
    }

    pub fn add(&self, other: &ComplexMatrix) -> Result<ComplexMatrix, JsValue> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(JsValue::from_str("Matrix dimensions must match"));
        }

        let mut result = ComplexMatrix::new(self.rows, self.cols);
        for i in 0..self.data.len() {
            result.data[i] = self.data[i].add(&other.data[i]);
        }
        Ok(result)
    }

    pub fn multiply(&self, other: &ComplexMatrix) -> Result<ComplexMatrix, JsValue> {
        if self.cols != other.rows {
            return Err(JsValue::from_str("Invalid matrix dimensions for multiplication"));
        }

        let mut result = ComplexMatrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = Complex::new(0.0, 0.0);
                for k in 0..self.cols {
                    sum = sum.add(&self.get(i, k).multiply(&other.get(k, j)));
                }
                result.set(i, j, &sum);
            }
        }
        Ok(result)
    }

    pub fn adjoint(&self) -> ComplexMatrix {
        let mut result = ComplexMatrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, &self.get(i, j).conjugate());
            }
        }
        result
    }

    pub fn trace(&self) -> Complex {
        let mut sum = Complex::new(0.0, 0.0);
        let min_dim = self.rows.min(self.cols);
        for i in 0..min_dim {
            sum = sum.add(&self.get(i, i));
        }
        sum
    }

    pub fn is_hermitian(&self) -> bool {
        if self.rows != self.cols {
            return false;
        }

        for i in 0..self.rows {
            for j in 0..self.cols {
                let elem = self.get(i, j);
                let conj_elem = self.get(j, i).conjugate();
                if (elem.real - conj_elem.real).abs() > 1e-10 || 
                   (elem.imag - conj_elem.imag).abs() > 1e-10 {
                    return false;
                }
            }
        }
        true
    }

    pub fn tensor_product(&self, other: &ComplexMatrix) -> ComplexMatrix {
        let new_rows = self.rows * other.rows;
        let new_cols = self.cols * other.cols;
        let mut result = ComplexMatrix::new(new_rows, new_cols);

        for i1 in 0..self.rows {
            for j1 in 0..self.cols {
                for i2 in 0..other.rows {
                    for j2 in 0..other.cols {
                        let prod = self.get(i1, j1).multiply(&other.get(i2, j2));
                        result.set(
                            i1 * other.rows + i2,
                            j1 * other.cols + j2,
                            &prod
                        );
                    }
                }
            }
        }
        result
    }

    pub fn matrix_exp(&self) -> Result<ComplexMatrix, JsValue> {
        if self.rows != self.cols {
            return Err(JsValue::from_str("Matrix must be square"));
        }

        let mut result = ComplexMatrix::identity(self.rows);
        let mut term = ComplexMatrix::identity(self.rows);
        let mut factorial = 1.0;

        for n in 1..20 {
            factorial *= n as f64;
            term = term.multiply(self)?;
            let scaled_term = term.scalar_multiply(1.0 / factorial);
            result = result.add(&scaled_term)?;
        }

        Ok(result)
    }

    pub fn scalar_multiply(&self, scalar: f64) -> ComplexMatrix {
        let mut result = ComplexMatrix::new(self.rows, self.cols);
        for i in 0..self.data.len() {
            result.data[i] = self.data[i].multiply(&Complex::new(scalar, 0.0));
        }
        result
    }

    pub fn from_vector(vector: Vec<Complex>) -> ComplexMatrix {
        let rows = vector.len();
        let mut matrix = ComplexMatrix::new(rows, 1);
        for (i, value) in vector.iter().enumerate() {
            matrix.set(i, 0, value);
        }
        matrix
    }

    pub fn to_vector(&self) -> Vec<Complex> {
        if self.cols != 1 {
            panic!("Matrix must have exactly one column to convert to vector");
        }
        self.data.clone()
    }

    #[wasm_bindgen]
    pub fn measure_qubit(&self, qubit_index: usize, num_qubits: usize) -> Result<JsValue, JsValue> {
        if qubit_index >= num_qubits {
            return Err(JsValue::from_str("Invalid qubit index"));
        }

        let size = 1 << num_qubits;
        if self.rows != size || self.cols != 1 {
            return Err(JsValue::from_str("Invalid state vector dimensions"));
        }

        let mut proj0 = ComplexMatrix::new(size, size);
        let mut proj1 = ComplexMatrix::new(size, size);

        for i in 0..size {
            let bit = (i >> qubit_index) & 1;
            if bit == 0 {
                proj0.set(i, i, &Complex::new(1.0, 0.0));
            } else {
                proj1.set(i, i, &Complex::new(1.0, 0.0));
            }
        }

        let obj = js_sys::Object::new();
        
        let proj0_obj = js_sys::Object::new();
        js_sys::Reflect::set(&proj0_obj, &"rows".into(), &JsValue::from(proj0.rows))?;
        js_sys::Reflect::set(&proj0_obj, &"cols".into(), &JsValue::from(proj0.cols))?;
        let data0 = js_sys::Array::new();
        for value in proj0.data {
            data0.push(&JsValue::from(value));
        }
        js_sys::Reflect::set(&proj0_obj, &"data".into(), &data0)?;
        
        let proj1_obj = js_sys::Object::new();
        js_sys::Reflect::set(&proj1_obj, &"rows".into(), &JsValue::from(proj1.rows))?;
        js_sys::Reflect::set(&proj1_obj, &"cols".into(), &JsValue::from(proj1.cols))?;
        let data1 = js_sys::Array::new();
        for value in proj1.data {
            data1.push(&JsValue::from(value));
        }
        js_sys::Reflect::set(&proj1_obj, &"data".into(), &data1)?;

        js_sys::Reflect::set(&obj, &"0".into(), &proj0_obj)?;
        js_sys::Reflect::set(&obj, &"1".into(), &proj1_obj)?;
        Ok(obj.into())
    }

    #[wasm_bindgen(getter)]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[wasm_bindgen(getter)]
    pub fn cols(&self) -> usize {
        self.cols
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<Complex> {
        self.data.clone()
    }
}

impl ComplexMatrix {
    pub fn internal_measure_qubit(&self, qubit_index: usize, num_qubits: usize) 
        -> Result<(ComplexMatrix, ComplexMatrix), String> {
        if qubit_index >= num_qubits {
            return Err("Invalid qubit index".to_string());
        }

        let size = 1 << num_qubits;
        if self.rows != size || self.cols != 1 {
            return Err("Invalid state vector dimensions".to_string());
        }

        let mut proj0 = ComplexMatrix::new(size, size);
        let mut proj1 = ComplexMatrix::new(size, size);

        for i in 0..size {
            let bit = (i >> qubit_index) & 1;
            if bit == 0 {
                proj0.set(i, i, &Complex::new(1.0, 0.0));
            } else {
                proj1.set(i, i, &Complex::new(1.0, 0.0));
            }
        }

        Ok((proj0, proj1))
    }
}

impl Mul<Vec<Complex>> for ComplexMatrix {
    type Output = Vec<Complex>;

    fn mul(self, rhs: Vec<Complex>) -> Self::Output {
        if self.cols != rhs.len() {
            panic!("Matrix columns must match vector length");
        }

        let mut result = vec![Complex::new(0.0, 0.0); self.rows];
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i] = result[i].add(&self.get(i, j).multiply(&rhs[j]));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let matrix = ComplexMatrix::new(2, 3);
        assert_eq!(matrix.rows, 2);
        assert_eq!(matrix.cols, 3);
        assert_eq!(matrix.data.len(), 6);
    }

    #[test]
    fn test_identity_matrix() {
        let matrix = ComplexMatrix::identity(2);
        assert_eq!(matrix.get(0, 0).real, 1.0);
        assert_eq!(matrix.get(0, 0).imag, 0.0);
        assert_eq!(matrix.get(1, 1).real, 1.0);
        assert_eq!(matrix.get(1, 1).imag, 0.0);
        assert_eq!(matrix.get(0, 1).real, 0.0);
        assert_eq!(matrix.get(1, 0).real, 0.0);
    }

    #[test]
    fn test_pauli_matrices() {
        // Test Pauli X
        let x = ComplexMatrix::pauli_x();
        assert_eq!(x.get(0, 1).real, 1.0);
        assert_eq!(x.get(1, 0).real, 1.0);
        assert_eq!(x.get(0, 0).real, 0.0);
        assert_eq!(x.get(1, 1).real, 0.0);

        // Test Pauli Y
        let y = ComplexMatrix::pauli_y();
        assert_eq!(y.get(0, 1).imag, -1.0);
        assert_eq!(y.get(1, 0).imag, 1.0);
        assert_eq!(y.get(0, 0).real, 0.0);
        assert_eq!(y.get(1, 1).real, 0.0);

        // Test Pauli Z
        let z = ComplexMatrix::pauli_z();
        assert_eq!(z.get(0, 0).real, 1.0);
        assert_eq!(z.get(1, 1).real, -1.0);
        assert_eq!(z.get(0, 1).real, 0.0);
        assert_eq!(z.get(1, 0).real, 0.0);
    }

    #[test]
    fn test_matrix_addition() -> Result<(), JsValue> {
        let mut a = ComplexMatrix::new(2, 2);
        let mut b = ComplexMatrix::new(2, 2);
        
        a.set(0, 0, &Complex::new(1.0, 0.0));
        b.set(0, 0, &Complex::new(2.0, 0.0));
        
        let c = a.add(&b)?;
        assert_eq!(c.get(0, 0).real, 3.0);
        
        Ok(())
    }

    #[test]
    fn test_matrix_multiplication() -> Result<(), JsValue> {
        let mut a = ComplexMatrix::new(2, 2);
        let mut b = ComplexMatrix::new(2, 2);
        
        a.set(0, 0, &Complex::new(1.0, 0.0));
        a.set(0, 1, &Complex::new(2.0, 0.0));
        b.set(0, 0, &Complex::new(3.0, 0.0));
        b.set(1, 0, &Complex::new(4.0, 0.0));
        
        let c = a.multiply(&b)?;
        assert_eq!(c.get(0, 0).real, 11.0);
        
        Ok(())
    }

    #[test]
    fn test_adjoint() {
        let mut matrix = ComplexMatrix::new(2, 2);
        matrix.set(0, 1, &Complex::new(1.0, 1.0));
        
        let adj = matrix.adjoint();
        assert_eq!(adj.get(1, 0).real, 1.0);
        assert_eq!(adj.get(1, 0).imag, -1.0);
    }

    #[test]
    fn test_trace() {
        let mut matrix = ComplexMatrix::new(2, 2);
        matrix.set(0, 0, &Complex::new(1.0, 0.0));
        matrix.set(1, 1, &Complex::new(2.0, 0.0));
        
        let tr = matrix.trace();
        assert_eq!(tr.real, 3.0);
        assert_eq!(tr.imag, 0.0);
    }

    #[test]
    fn test_hermitian() {
        let mut matrix = ComplexMatrix::new(2, 2);
        matrix.set(0, 0, &Complex::new(1.0, 0.0));
        matrix.set(0, 1, &Complex::new(1.0, -1.0));
        matrix.set(1, 0, &Complex::new(1.0, 1.0));
        matrix.set(1, 1, &Complex::new(2.0, 0.0));
        
        assert!(matrix.is_hermitian());
    }

    #[test]
    fn test_tensor_product() {
        let mut a = ComplexMatrix::new(2, 2);
        let mut b = ComplexMatrix::new(2, 2);
        
        a.set(0, 0, &Complex::new(1.0, 0.0));
        b.set(0, 0, &Complex::new(2.0, 0.0));
        
        let c = a.tensor_product(&b);
        assert_eq!(c.rows, 4);
        assert_eq!(c.cols, 4);
        assert_eq!(c.get(0, 0).real, 2.0);
    }

    #[test]
    fn test_matrix_exp() -> Result<(), JsValue> {
        let mut matrix = ComplexMatrix::new(2, 2);
        matrix.set(0, 1, &Complex::new(1.0, 0.0));
        matrix.set(1, 0, &Complex::new(1.0, 0.0));
        
        let exp = matrix.matrix_exp()?;
        // For this particular matrix, exp should be close to cosh(1) * I + sinh(1) * X
        let cosh1 = 1.5430806348152437;  // cosh(1)
        let sinh1 = 1.1752011936438014;  // sinh(1)
        
        assert!((exp.get(0, 0).real - cosh1).abs() < 0.01);
        assert!((exp.get(0, 1).real - sinh1).abs() < 0.01);
        
        Ok(())
    }

    #[test]
    fn test_matrix_vector_multiplication() {
        let mut matrix = ComplexMatrix::new(2, 2);
        matrix.set(0, 0, &Complex::new(1.0, 0.0));
        matrix.set(0, 1, &Complex::new(2.0, 0.0));
        matrix.set(1, 0, &Complex::new(3.0, 0.0));
        matrix.set(1, 1, &Complex::new(4.0, 0.0));
        
        let vector = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0)
        ];
        
        let result = matrix * vector;
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].real, 5.0);
        assert_eq!(result[0].imag, 0.0);
        assert_eq!(result[1].real, 11.0);
        assert_eq!(result[1].imag, 0.0);
    }

    #[test]
    fn test_internal_measure_qubit() {
        let mut state = ComplexMatrix::new(4, 1);  // 2-qubit state
        state.set(0, 0, &Complex::new(1.0, 0.0));  // |00⟩ state
        
        let result = state.internal_measure_qubit(0, 2);
        assert!(result.is_ok());
        
        let (proj0, proj1) = result.unwrap();
        
        // Check dimensions
        assert_eq!(proj0.rows, 4);
        assert_eq!(proj0.cols, 4);
        assert_eq!(proj1.rows, 4);
        assert_eq!(proj1.cols, 4);
        
        // Check projector properties
        assert_eq!(proj0.get(0, 0).real, 1.0);
        assert_eq!(proj1.get(1, 1).real, 1.0);
    }
}
