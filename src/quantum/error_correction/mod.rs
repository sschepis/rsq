use wasm_bindgen::prelude::*;
use super::circuit::QuantumCircuit;
use crate::quantum::core::matrix::ComplexMatrix;
use crate::quantum::core::complex::Complex;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ErrorCorrection {
    code_type: String,
    num_physical_qubits: usize,
    num_logical_qubits: usize,
}

#[derive(Debug)]
pub enum TestError {
    InsufficientQubits,
    UnsupportedCode,
    Other(String),
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
impl ErrorCorrection {
    #[wasm_bindgen(constructor)]
    pub fn new(code_type: &str) -> ErrorCorrection {
        let (physical, logical) = match code_type {
            "bit-flip" => (3, 1),
            "phase-flip" => (3, 1),
            "shor" => (9, 1),
            "steane" => (7, 1),
            _ => (3, 1),
        };
        
        ErrorCorrection {
            code_type: code_type.to_string(),
            num_physical_qubits: physical,
            num_logical_qubits: logical,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn code_type(&self) -> String {
        self.code_type.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn num_physical_qubits(&self) -> usize {
        self.num_physical_qubits
    }

    #[wasm_bindgen(getter)]
    pub fn num_logical_qubits(&self) -> usize {
        self.num_logical_qubits
    }

    #[cfg(target_arch = "wasm32")]
    pub fn encode(&self, circuit: &mut QuantumCircuit, qubit: usize) -> Result<(), JsValue> {
        match self.code_type.as_str() {
            "bit-flip" => circuit.encode_bit_flip(qubit),
            "steane" => circuit.encode_steane(qubit),
            _ => Err(JsValue::from_str("Unsupported error correction code")),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn syndrome_measurement(&self, circuit: &mut QuantumCircuit) -> Result<JsValue, JsValue> {
        circuit.detect_errors()
    }
}

impl ErrorCorrection {
    #[cfg(test)]
    pub fn test_encode(&self, circuit: &mut QuantumCircuit, qubit: usize) -> Result<(), TestError> {
        match self.code_type.as_str() {
            "bit-flip" => circuit.encode_bit_flip(qubit)
                .map_err(|_| TestError::Other("Bit flip encoding failed".to_string())),
            "steane" => circuit.encode_steane(qubit)
                .map_err(|_| TestError::Other("Steane encoding failed".to_string())),
            _ => Err(TestError::UnsupportedCode),
        }
    }

    #[cfg(test)]
    pub fn test_syndrome_measurement(&self, circuit: &mut QuantumCircuit) -> Result<bool, TestError> {
        circuit.detect_errors()
            .map(|_| true)
            .map_err(|_| TestError::Other("Syndrome measurement failed".to_string()))
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SurfaceCode {
    size: usize,
    data_qubits: Vec<(usize, usize)>,
    measure_x_qubits: Vec<(usize, usize)>,
    measure_z_qubits: Vec<(usize, usize)>,
}

#[wasm_bindgen]
impl SurfaceCode {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> SurfaceCode {
        let mut data_qubits = Vec::new();
        let mut measure_x_qubits = Vec::new();
        let mut measure_z_qubits = Vec::new();
        
        // Place qubits in a checkerboard pattern
        for i in 0..size {
            for j in 0..size {
                if (i + j) % 2 == 0 {  // Data qubits on even-sum coordinates
                    data_qubits.push((i, j));
                } else if i % 2 == 0 {
                    measure_x_qubits.push((i, j));
                } else {
                    measure_z_qubits.push((i, j));
                }
            }
        }
        
        SurfaceCode {
            size,
            data_qubits,
            measure_x_qubits,
            measure_z_qubits,
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn stabilizer_measurement(&self, circuit: &mut QuantumCircuit) -> Result<(), JsValue> {
        self.internal_stabilizer_measurement(circuit)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }
}

impl SurfaceCode {
    fn internal_stabilizer_measurement(&self, circuit: &mut QuantumCircuit) -> Result<(), TestError> {
        // X-type stabilizers
        for &(i, j) in &self.measure_x_qubits {
            let neighbors = self.get_data_neighbors(i, j);
            circuit.add_gate("H", vec![self.qubit_index(i, j)], None);
            
            for &neighbor in &neighbors {
                circuit.add_gate("CNOT", vec![self.qubit_index(i, j), self.qubit_index(neighbor.0, neighbor.1)], None);
            }
            
            circuit.add_gate("H", vec![self.qubit_index(i, j)], None);
        }
        
        // Z-type stabilizers
        for &(i, j) in &self.measure_z_qubits {
            let neighbors = self.get_data_neighbors(i, j);
            
            for &neighbor in &neighbors {
                circuit.add_gate("CNOT", vec![self.qubit_index(neighbor.0, neighbor.1), self.qubit_index(i, j)], None);
            }
        }
        Ok(())
    }

    pub fn get_data_neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        
        // For X stabilizers (even rows), only check horizontal neighbors
        if i % 2 == 0 {
            if j > 0 {
                neighbors.push((i, j-1));
            }
            if j + 1 < self.size {
                neighbors.push((i, j+1));
            }
        }
        // For Z stabilizers (odd rows), only check vertical neighbors
        else {
            if i > 0 {
                neighbors.push((i-1, j));
            }
            if i + 1 < self.size {
                neighbors.push((i+1, j));
            }
        }
        
        neighbors
    }

    fn qubit_index(&self, i: usize, j: usize) -> usize {
        i * self.size + j
    }

    #[cfg(test)]
    pub fn test_get_data_qubits(&self) -> Vec<(usize, usize)> {
        self.data_qubits.clone()
    }

    #[cfg(test)]
    pub fn test_get_measure_x_qubits(&self) -> Vec<(usize, usize)> {
        self.measure_x_qubits.clone()
    }

    #[cfg(test)]
    pub fn test_get_measure_z_qubits(&self) -> Vec<(usize, usize)> {
        self.measure_z_qubits.clone()
    }

    #[cfg(test)]
    pub fn test_stabilizer_measurement(&self, circuit: &mut QuantumCircuit) -> Result<(), TestError> {
        self.internal_stabilizer_measurement(circuit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_code_neighbors() {
        let code = SurfaceCode::new(3);
        
        // Test X stabilizer neighbors (measurement qubit at (0,1))
        let x_stabilizer_neighbors = code.get_data_neighbors(0, 1);
        assert_eq!(x_stabilizer_neighbors.len(), 2);  // Only has 2 data qubit neighbors
        assert!(x_stabilizer_neighbors.contains(&(0, 0)));  // Data qubit to the left
        assert!(x_stabilizer_neighbors.contains(&(0, 2)));  // Data qubit to the right
        
        // Test Z stabilizer neighbors (measurement qubit at (1,0))
        let z_stabilizer_neighbors = code.get_data_neighbors(1, 0);
        assert_eq!(z_stabilizer_neighbors.len(), 2);  // Only has 2 data qubit neighbors
        assert!(z_stabilizer_neighbors.contains(&(0, 0)));  // Data qubit above
        assert!(z_stabilizer_neighbors.contains(&(2, 0)));  // Data qubit below
    }

    #[test]
    fn test_error_correction_creation() {
        // Test bit-flip code
        let bit_flip = ErrorCorrection::new("bit-flip");
        assert_eq!(bit_flip.code_type(), "bit-flip");
        assert_eq!(bit_flip.num_physical_qubits(), 3);
        assert_eq!(bit_flip.num_logical_qubits(), 1);

        // Test phase-flip code
        let phase_flip = ErrorCorrection::new("phase-flip");
        assert_eq!(phase_flip.code_type(), "phase-flip");
        assert_eq!(phase_flip.num_physical_qubits(), 3);
        assert_eq!(phase_flip.num_logical_qubits(), 1);

        // Test Shor code
        let shor = ErrorCorrection::new("shor");
        assert_eq!(shor.code_type(), "shor");
        assert_eq!(shor.num_physical_qubits(), 9);
        assert_eq!(shor.num_logical_qubits(), 1);

        // Test Steane code
        let steane = ErrorCorrection::new("steane");
        assert_eq!(steane.code_type(), "steane");
        assert_eq!(steane.num_physical_qubits(), 7);
        assert_eq!(steane.num_logical_qubits(), 1);

        // Test default case
        let default = ErrorCorrection::new("unknown");
        assert_eq!(default.num_physical_qubits(), 3);
        assert_eq!(default.num_logical_qubits(), 1);
    }

    #[test]
    fn test_error_correction_encoding() -> Result<(), TestError> {
        let mut circuit = QuantumCircuit::new(4);  // Need extra qubit for encoding
        
        // Test bit-flip encoding
        let bit_flip = ErrorCorrection::new("bit-flip");
        bit_flip.test_encode(&mut circuit, 0)?;
        
        // Test Steane encoding
        let mut large_circuit = QuantumCircuit::new(8);  // Need 7+1 qubits for Steane
        let steane = ErrorCorrection::new("steane");
        steane.test_encode(&mut large_circuit, 0)?;
        
        // Test unsupported code
        let unsupported = ErrorCorrection::new("unsupported");
        assert!(matches!(
            unsupported.test_encode(&mut circuit, 0),
            Err(TestError::UnsupportedCode)
        ));
        
        Ok(())
    }

    #[test]
    fn test_surface_code_creation() {
        let code = SurfaceCode::new(3);
        
        // Test data qubit placement - data qubits are on even-sum coordinates
        let data_qubits = code.test_get_data_qubits();
        assert!(data_qubits.contains(&(0, 0)));  // (0+0)%2 == 0
        assert!(data_qubits.contains(&(0, 2)));  // (0+2)%2 == 0
        assert!(data_qubits.contains(&(1, 1)));  // (1+1)%2 == 0
        assert!(data_qubits.contains(&(2, 0)));  // (2+0)%2 == 0
        assert!(data_qubits.contains(&(2, 2)));  // (2+2)%2 == 0
        
        // Test X measurement qubit placement - on even rows, odd columns
        let x_qubits = code.test_get_measure_x_qubits();
        assert!(x_qubits.contains(&(0, 1)));
        assert!(x_qubits.contains(&(2, 1)));
        
        // Test Z measurement qubit placement - on odd rows
        let z_qubits = code.test_get_measure_z_qubits();
        assert!(z_qubits.contains(&(1, 0)));
        assert!(z_qubits.contains(&(1, 2)));
    }

    #[test]
    fn test_surface_code_stabilizer_measurement() -> Result<(), TestError> {
        let code = SurfaceCode::new(3);
        let mut circuit = QuantumCircuit::new(9);  // 3x3 grid
        
        // Test stabilizer measurement circuit construction
        code.test_stabilizer_measurement(&mut circuit)?;
        
        // Verify that gates were added
        assert!(circuit.get_num_gates() > 0);
        
        Ok(())
    }

    #[test]
    fn test_qubit_indexing() {
        let code = SurfaceCode::new(3);
        
        // Test various positions
        assert_eq!(code.qubit_index(0, 0), 0);
        assert_eq!(code.qubit_index(0, 1), 1);
        assert_eq!(code.qubit_index(1, 0), 3);
        assert_eq!(code.qubit_index(2, 2), 8);
    }
}
