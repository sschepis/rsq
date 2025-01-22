use std::error::Error;
use crate::quantum::core::complex::Complex;
use crate::quantum::core::matrix::ComplexMatrix;

pub struct QuantumCircuit {
    state: ComplexMatrix,
    num_qubits: usize,
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        let mut state = ComplexMatrix::new(2usize.pow(num_qubits as u32), 1);
        state.set(0, 0, Complex::new(1.0, 0.0));
        
        QuantumCircuit {
            state,
            num_qubits,
        }
    }

    pub fn add_controlled_u(&mut self, control: usize, target: usize, _unitary: &ComplexMatrix) -> Result<(), Box<dyn Error>> {
        if control >= self.num_qubits || target >= self.num_qubits {
            return Err("Invalid qubit indices".into());
        }
        // TODO: Implement controlled unitary operation
        Ok(())
    }

    pub fn add_hadamard(&mut self, qubit: usize) -> Result<(), Box<dyn Error>> {
        if qubit >= self.num_qubits {
            return Err("Invalid qubit index".into());
        }
        
        let mut hadamard = ComplexMatrix::new(2, 2);
        let factor = 1.0 / 2.0_f64.sqrt();
        hadamard.set(0, 0, Complex::new(factor, 0.0));
        hadamard.set(0, 1, Complex::new(factor, 0.0));
        hadamard.set(1, 0, Complex::new(factor, 0.0));
        hadamard.set(1, 1, Complex::new(-factor, 0.0));
        
        self.apply_single_qubit_gate(qubit, &hadamard)
    }

    pub fn add_phase(&mut self, qubit: usize, phi: f64) -> Result<(), Box<dyn Error>> {
        if qubit >= self.num_qubits {
            return Err("Invalid qubit index".into());
        }
        
        let mut phase = ComplexMatrix::new(2, 2);
        phase.set(0, 0, Complex::new(1.0, 0.0));
        phase.set(1, 1, Complex::new(phi.cos(), phi.sin()));
        
        self.apply_single_qubit_gate(qubit, &phase)
    }

    fn error_probability(&self, _qubit: usize) -> f64 {
        // TODO: Implement error probability calculation
        0.0
    }

    pub fn add_pauli_x(&mut self, qubit: usize) -> Result<(), Box<dyn Error>> {
        if qubit >= self.num_qubits {
            return Err("Invalid qubit index".into());
        }
        
        let pauli_x = ComplexMatrix::pauli_x();
        self.apply_single_qubit_gate(qubit, &pauli_x)
    }

    pub fn add_pauli_y(&mut self, qubit: usize) -> Result<(), Box<dyn Error>> {
        if qubit >= self.num_qubits {
            return Err("Invalid qubit index".into());
        }
        
        let pauli_y = ComplexMatrix::pauli_y();
        self.apply_single_qubit_gate(qubit, &pauli_y)
    }

    pub fn add_pauli_z(&mut self, qubit: usize) -> Result<(), Box<dyn Error>> {
        if qubit >= self.num_qubits {
            return Err("Invalid qubit index".into());
        }
        
        let pauli_z = ComplexMatrix::pauli_z();
        self.apply_single_qubit_gate(qubit, &pauli_z)
    }

    pub fn add_cnot(&mut self, control: usize, target: usize) -> Result<(), Box<dyn Error>> {
        if control >= self.num_qubits || target >= self.num_qubits {
            return Err("Invalid qubit indices".into());
        }
        
        let pauli_x = ComplexMatrix::pauli_x();
        self.apply_controlled_gate(control, target, &pauli_x)
    }

    fn apply_single_qubit_gate(&mut self, qubit: usize, _gate: &ComplexMatrix) -> Result<(), Box<dyn Error>> {
        if qubit >= self.num_qubits {
            return Err("Invalid qubit index".into());
        }
        // TODO: Implement single qubit gate application
        Ok(())
    }

    fn apply_controlled_gate(&mut self, control: usize, target: usize, _gate: &ComplexMatrix) -> Result<(), Box<dyn Error>> {
        if control >= self.num_qubits || target >= self.num_qubits {
            return Err("Invalid qubit indices".into());
        }
        // TODO: Implement controlled gate application
        Ok(())
    }

    pub fn get_state(&self) -> &ComplexMatrix {
        &self.state
    }

    pub fn get_num_qubits(&self) -> usize {
        self.num_qubits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_circuit() {
        let circuit = QuantumCircuit::new(2);
        assert_eq!(circuit.get_num_qubits(), 2);
        assert_eq!(circuit.get_state().rows(), 4);
        assert_eq!(circuit.get_state().cols(), 1);
    }

    #[test]
    fn test_hadamard() {
        let mut circuit = QuantumCircuit::new(1);
        assert!(circuit.add_hadamard(0).is_ok());
        assert!(circuit.add_hadamard(1).is_err());
    }

    #[test]
    fn test_cnot() {
        let mut circuit = QuantumCircuit::new(2);
        assert!(circuit.add_cnot(0, 1).is_ok());
        assert!(circuit.add_cnot(0, 2).is_err());
    }

    #[test]
    fn test_pauli_gates() {
        let mut circuit = QuantumCircuit::new(1);
        assert!(circuit.add_pauli_x(0).is_ok());
        assert!(circuit.add_pauli_y(0).is_ok());
        assert!(circuit.add_pauli_z(0).is_ok());
        assert!(circuit.add_pauli_x(1).is_err());
    }
}
