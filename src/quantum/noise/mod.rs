use wasm_bindgen::prelude::*;
use crate::quantum::core::matrix::ComplexMatrix;
use crate::quantum::core::complex::Complex;
use rand::Rng;

#[wasm_bindgen]
#[derive(Debug)]
pub struct NoiseModel {
    depolarizing_prob: f64,
    bit_flip_prob: f64,
    phase_flip_prob: f64,
    amplitude_damping_prob: f64,
}

#[wasm_bindgen]
impl NoiseModel {
    #[wasm_bindgen(constructor)]
    pub fn new(
        depolarizing: f64,
        bit_flip: f64,
        phase_flip: f64,
        amplitude_damping: f64
    ) -> NoiseModel {
        NoiseModel {
            depolarizing_prob: depolarizing,
            bit_flip_prob: bit_flip,
            phase_flip_prob: phase_flip,
            amplitude_damping_prob: amplitude_damping,
        }
    }

    pub fn generate_noise_matrix(&self) -> Result<ComplexMatrix, JsValue> {
        let mut noise_matrix = ComplexMatrix::identity(2);
        
        // Apply depolarizing noise
        if self.depolarizing_prob > 0.0 {
            let pauli_x = ComplexMatrix::pauli_x();
            let pauli_y = ComplexMatrix::pauli_y();
            let pauli_z = ComplexMatrix::pauli_z();
            
            let depolarizing = pauli_x
                .add(&pauli_y)?
                .add(&pauli_z)?
                .scalar_multiply(self.depolarizing_prob / 3.0);
            
            noise_matrix = noise_matrix.add(&depolarizing)?;
        }
        
        // Apply bit flip noise
        if self.bit_flip_prob > 0.0 {
            let bit_flip = ComplexMatrix::pauli_x()
                .scalar_multiply(self.bit_flip_prob);
            noise_matrix = noise_matrix.add(&bit_flip)?;
        }
        
        // Apply phase flip noise
        if self.phase_flip_prob > 0.0 {
            let phase_flip = ComplexMatrix::pauli_z()
                .scalar_multiply(self.phase_flip_prob);
            noise_matrix = noise_matrix.add(&phase_flip)?;
        }
        
        // Apply amplitude damping
        if self.amplitude_damping_prob > 0.0 {
            let gamma = self.amplitude_damping_prob;
            let mut damping = ComplexMatrix::new(2, 2);
            damping.set(0, 0, &Complex::new(1.0, 0.0));
            damping.set(1, 1, &Complex::new((1.0 - gamma).sqrt(), 0.0));
            noise_matrix = noise_matrix.multiply(&damping)?;
        }
        
        Ok(noise_matrix)
    }

    pub fn apply_noise(&self, state: &mut ComplexMatrix) -> Result<(), JsValue> {
        let noise_matrix = self.generate_noise_matrix()?;
        state.multiply(&noise_matrix)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn test_noise_model_creation() {
        let model = NoiseModel::new(0.1, 0.2, 0.3, 0.4);
        assert!((model.depolarizing_prob - 0.1).abs() < EPSILON);
        assert!((model.bit_flip_prob - 0.2).abs() < EPSILON);
        assert!((model.phase_flip_prob - 0.3).abs() < EPSILON);
        assert!((model.amplitude_damping_prob - 0.4).abs() < EPSILON);
    }

    #[test]
    fn test_noise_matrix_generation() -> Result<(), JsValue> {
        let model = NoiseModel::new(0.1, 0.2, 0.3, 0.4);
        let noise_matrix = model.generate_noise_matrix()?;
        
        // Verify matrix dimensions
        assert_eq!(noise_matrix.rows(), 2);
        assert_eq!(noise_matrix.cols(), 2);
        
        Ok(())
    }
}
