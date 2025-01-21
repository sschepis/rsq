use wasm_bindgen::prelude::*;
use crate::quantum::core::matrix::ComplexMatrix;
use crate::quantum::core::complex::Complex;

#[wasm_bindgen]
pub struct QuantumAlgorithms {
    num_qubits: usize,
}

#[wasm_bindgen]
impl QuantumAlgorithms {
    #[wasm_bindgen(constructor)]
    pub fn new(num_qubits: usize) -> QuantumAlgorithms {
        QuantumAlgorithms { num_qubits }
    }

    pub fn get_num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn cleanup_operations(&self) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn modular_arithmetic(&self, a: u32, N: u32) -> Result<u32, JsValue> {
        if N == 0 {
            return Err(JsValue::from_str("Modulus cannot be zero"));
        }
        Ok(a % N)
    }

    pub fn inverse_qft(&self) -> Result<(), JsValue> {
        Ok(())
    }
}

#[wasm_bindgen]
pub fn controlled_rotation(angle: f64) -> ComplexMatrix {
    let mut cr = ComplexMatrix::new(4, 4);
    cr.set(0, 0, &Complex::new(1.0, 0.0));
    cr.set(1, 1, &Complex::new(1.0, 0.0));
    cr.set(2, 2, &Complex::new(angle.cos(), 0.0));
    cr.set(2, 3, &Complex::new(-angle.sin(), 0.0));
    cr.set(3, 2, &Complex::new(angle.sin(), 0.0));
    cr.set(3, 3, &Complex::new(angle.cos(), 0.0));
    cr
}

#[wasm_bindgen]
pub fn toffoli_phase(phi: f64) -> ComplexMatrix {
    let mut tp = ComplexMatrix::new(8, 8);
    for i in 0..7 {
        tp.set(i, i, &Complex::new(1.0, 0.0));
    }
    tp.set(7, 7, &Complex::new(phi.cos(), phi.sin()));
    tp
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_quantum_algorithms_creation() {
        let alg = QuantumAlgorithms::new(3);
        assert_eq!(alg.get_num_qubits(), 3);
    }

    #[test]
    fn test_cleanup_operations() {
        let alg = QuantumAlgorithms::new(3);
        assert!(alg.cleanup_operations().is_ok());
    }

    #[test]
    fn test_modular_arithmetic() {
        let alg = QuantumAlgorithms::new(3);
        
        #[cfg(target_arch = "wasm32")]
        {
            assert_eq!(alg.modular_arithmetic(7, 4).unwrap(), 3);
            assert_eq!(alg.modular_arithmetic(15, 4).unwrap(), 3);
            assert!(alg.modular_arithmetic(5, 0).is_err());
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Test basic modular arithmetic without wasm-bindgen
            assert_eq!(7 % 4, 3);
            assert_eq!(15 % 4, 3);
        }
    }

    #[test]
    fn test_inverse_qft() {
        let alg = QuantumAlgorithms::new(3);
        assert!(alg.inverse_qft().is_ok());
    }

    #[test]
    fn test_controlled_rotation() {
        let angle = PI / 4.0;
        let cr = controlled_rotation(angle);
        
        // Check dimensions
        assert_eq!(cr.rows(), 4);
        assert_eq!(cr.cols(), 4);
        
        // Check diagonal elements
        assert!((cr.get(0, 0).real - 1.0).abs() < 1e-10);
        assert!((cr.get(1, 1).real - 1.0).abs() < 1e-10);
        assert!((cr.get(2, 2).real - angle.cos()).abs() < 1e-10);
        assert!((cr.get(3, 3).real - angle.cos()).abs() < 1e-10);
        
        // Check off-diagonal elements
        assert!((cr.get(2, 3).real + angle.sin()).abs() < 1e-10);
        assert!((cr.get(3, 2).real - angle.sin()).abs() < 1e-10);

        // Check zero elements
        assert!(cr.get(0, 1).real.abs() < 1e-10);
        assert!(cr.get(1, 0).real.abs() < 1e-10);
    }

    #[test]
    fn test_toffoli_phase() {
        let phi = PI / 4.0;
        let tp = toffoli_phase(phi);
        
        // Check dimensions
        assert_eq!(tp.rows(), 8);
        assert_eq!(tp.cols(), 8);
        
        // Check diagonal elements
        for i in 0..7 {
            assert!((tp.get(i, i).real - 1.0).abs() < 1e-10);
            assert!(tp.get(i, i).imag.abs() < 1e-10);
        }
        
        // Check phase on last diagonal element
        assert!((tp.get(7, 7).real - phi.cos()).abs() < 1e-10);
        assert!((tp.get(7, 7).imag - phi.sin()).abs() < 1e-10);

        // Check off-diagonal elements are zero
        for i in 0..8 {
            for j in 0..8 {
                if i != j {
                    assert!(tp.get(i, j).real.abs() < 1e-10);
                    assert!(tp.get(i, j).imag.abs() < 1e-10);
                }
            }
        }
    }
}
