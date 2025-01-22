mod prime_quantum;
pub use prime_quantum::PrimeQuantumState;

use std::error::Error;
use std::fmt;
use rand::Rng;
use crate::quantum::core::complex::Complex;
use crate::quantum::core::matrix::ComplexMatrix;

#[derive(Debug)]
pub enum QuantumError {
    InvalidState,
    NoiseApplicationFailed,
    InvalidMeasurement,
    MatrixOperationFailed,
}

impl fmt::Display for QuantumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuantumError::InvalidState => write!(f, "Invalid quantum state"),
            QuantumError::NoiseApplicationFailed => write!(f, "Failed to apply noise channel"),
            QuantumError::InvalidMeasurement => write!(f, "Invalid measurement basis"),
            QuantumError::MatrixOperationFailed => write!(f, "Matrix operation failed"),
        }
    }
}

impl Error for QuantumError {}

#[derive(Debug, Clone)]
pub struct QuantumState {
    amplitudes: Vec<Complex>,
    basis_size: usize,
}

impl QuantumState {
    pub fn new(basis_size: usize) -> Self {
        let mut amplitudes = vec![Complex::new(0.0, 0.0); basis_size];
        amplitudes[0] = Complex::new(1.0, 0.0); // Initialize to |0⟩ state
        
        QuantumState {
            amplitudes,
            basis_size,
        }
    }

    pub fn evolve(&mut self, evolution_matrix: &ComplexMatrix) -> Result<(), QuantumError> {
        let state_matrix = ComplexMatrix::from_vector(self.amplitudes.clone());
        let result = evolution_matrix.multiply(&state_matrix)
            .map_err(|_| QuantumError::MatrixOperationFailed)?;
        
        let new_amplitudes = result.multiply_vector(&vec![Complex::new(1.0, 0.0)])
            .map_err(|_| QuantumError::MatrixOperationFailed)?;
            
        self.amplitudes = new_amplitudes;
        Ok(())
    }

    pub fn apply_noise(&mut self, noise_matrix: &ComplexMatrix) -> Result<(), QuantumError> {
        let state_matrix = ComplexMatrix::from_vector(self.amplitudes.clone());
        let noisy_state = noise_matrix.multiply(&state_matrix)
            .map_err(|_| QuantumError::NoiseApplicationFailed)?;
        
        let new_amplitudes = noisy_state.multiply_vector(&vec![Complex::new(1.0, 0.0)])
            .map_err(|_| QuantumError::NoiseApplicationFailed)?;
            
        self.amplitudes = new_amplitudes;
        Ok(())
    }

    pub fn measure(&self) -> Result<usize, QuantumError> {
        let mut probabilities = Vec::with_capacity(self.basis_size);
        let mut total_prob = 0.0;
        
        for amp in &self.amplitudes {
            let prob = amp.norm_sqr();
            probabilities.push(prob);
            total_prob += prob;
        }
        
        // Normalize probabilities
        if total_prob <= 0.0 {
            return Err(QuantumError::InvalidState);
        }
        
        for prob in &mut probabilities {
            *prob /= total_prob;
        }
        
        // Generate random number
        let mut rng = rand::thread_rng();
        let r: f64 = rng.gen();
        
        // Find measurement outcome
        let mut cumulative = 0.0;
        for (i, &prob) in probabilities.iter().enumerate() {
            cumulative += prob;
            if r <= cumulative {
                return Ok(i);
            }
        }
        
        Err(QuantumError::InvalidMeasurement)
    }

    pub fn get_amplitudes(&self) -> &[Complex] {
        &self.amplitudes
    }

    pub fn set_amplitudes(&mut self, new_amplitudes: Vec<Complex>) -> Result<(), QuantumError> {
        if new_amplitudes.len() != self.basis_size {
            return Err(QuantumError::InvalidState);
        }
        
        // Verify normalization
        let total_prob: f64 = new_amplitudes.iter()
            .map(|amp| amp.norm_sqr())
            .sum();
            
        if (total_prob - 1.0).abs() > 1e-10 {
            return Err(QuantumError::InvalidState);
        }
        
        self.amplitudes = new_amplitudes;
        Ok(())
    }

    pub fn to_matrix(&self) -> ComplexMatrix {
        ComplexMatrix::from_vector(self.amplitudes.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state() {
        let state = QuantumState::new(2);
        assert_eq!(state.amplitudes.len(), 2);
        assert_eq!(state.amplitudes[0].real, 1.0);
        assert_eq!(state.amplitudes[0].imag, 0.0);
        assert_eq!(state.amplitudes[1].real, 0.0);
        assert_eq!(state.amplitudes[1].imag, 0.0);
    }

    #[test]
    fn test_measure() {
        let state = QuantumState::new(2);
        let result = state.measure();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_invalid_amplitudes() {
        let mut state = QuantumState::new(2);
        let invalid_amplitudes = vec![Complex::new(1.0, 0.0)]; // Wrong size
        assert!(state.set_amplitudes(invalid_amplitudes).is_err());
    }

    #[test]
    fn test_evolution() {
        let mut state = QuantumState::new(2);
        let evolution = ComplexMatrix::identity(2);
        assert!(state.evolve(&evolution).is_ok());
    }
}
