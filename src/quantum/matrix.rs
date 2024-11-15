use wasm_bindgen::prelude::*;
use super::complex::Complex;

#[wasm_bindgen]
#[derive(Clone)]
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

    pub fn measure_qubit(&self, qubit_index: usize, num_qubits: usize) 
        -> Result<JsValue, JsValue> {
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
        
        // Convert proj0 to JsValue
        let proj0_obj = js_sys::Object::new();
        js_sys::Reflect::set(&proj0_obj, &"rows".into(), &JsValue::from(proj0.rows))?;
        js_sys::Reflect::set(&proj0_obj, &"cols".into(), &JsValue::from(proj0.cols))?;
        let data0 = js_sys::Array::new();
        for value in proj0.data {
            data0.push(&JsValue::from(value));
        }
        js_sys::Reflect::set(&proj0_obj, &"data".into(), &data0)?;
        
        // Convert proj1 to JsValue
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
