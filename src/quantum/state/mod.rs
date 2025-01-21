use std::fmt;
use crate::quantum::core::complex::Complex;
use crate::quantum::core::matrix::ComplexMatrix;
use crate::quantum::noise::NoiseModel;
use crate::quantum::hamiltonian::Hamiltonian;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub enum BasisType {
    Computational,
    Prime,
    Custom(Vec<Vec<Complex>>)
}

#[derive(Debug)]
pub struct StateMetadata {
    pub creation_time: f64,
    pub noise_profile: Option<NoiseModel>
}

pub struct QuantumState {
    amplitudes: Vec<Complex>,
    basis: BasisType,
    metadata: StateMetadata
}

impl QuantumState {
    pub fn new(dimension: usize, basis: BasisType) -> Self {
        let norm = 1.0 / (dimension as f64).sqrt();
        QuantumState {
            amplitudes: vec![Complex::new(norm, 0.0); dimension],
            basis: basis.clone(),
            metadata: StateMetadata {
                creation_time: 0.0,
                noise_profile: None
            }
        }
    }

    pub fn evolve(&mut self, time: f64, hamiltonian: &Hamiltonian) {
        let evolution_matrix = hamiltonian.matrix_representation(time);
        let new_state = evolution_matrix * self.amplitudes.clone();
        self.amplitudes = new_state;
    }

    pub fn apply_noise(&mut self, noise_model: &NoiseModel) -> Result<(), JsValue> {
        let noise_matrix = noise_model.generate_noise_matrix()?;
        let state_matrix = ComplexMatrix::from_vector(self.amplitudes.clone());
        let noisy_state = noise_matrix.multiply(&state_matrix)?;
        self.amplitudes = noisy_state.to_vector();
        Ok(())
    }
}

impl fmt::Display for QuantumState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QuantumState({} qubits)", self.amplitudes.len())
    }
}

#[derive(Debug)]
pub enum QuantumError {
    InvalidDimension,
    BasisMismatch,
    NoiseApplicationFailed
}

impl fmt::Display for QuantumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuantumError::InvalidDimension => write!(f, "Invalid dimension for quantum state"),
            QuantumError::BasisMismatch => write!(f, "Basis mismatch in operation"),
            QuantumError::NoiseApplicationFailed => write!(f, "Failed to apply noise model"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_initialization() {
        let state = QuantumState::new(2, BasisType::Computational);
        assert_eq!(state.amplitudes.len(), 2);
        assert!(state.amplitudes[0].norm() > 0.0);
    }

    #[test]
    fn test_state_display() {
        let state = QuantumState::new(3, BasisType::Prime);
        assert_eq!(format!("{}", state), "QuantumState(3 qubits)");
    }
}
