use wasm_bindgen::prelude::*;
use super::circuit::QuantumCircuit;

#[wasm_bindgen]
pub struct ErrorCorrection {
    code_type: String,
    num_physical_qubits: usize,
    num_logical_qubits: usize,
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

    pub fn encode(&self, circuit: &mut QuantumCircuit, qubit: usize) -> Result<(), JsValue> {
        match self.code_type.as_str() {
            "bit-flip" => circuit.encode_bit_flip(qubit),
            "steane" => circuit.encode_steane(qubit),
            _ => Err(JsValue::from_str("Unsupported error correction code")),
        }
    }

    pub fn syndrome_measurement(&self, circuit: &mut QuantumCircuit) -> Result<JsValue, JsValue> {
        circuit.detect_errors()
    }
}

#[wasm_bindgen]
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
        
        for i in 0..size {
            for j in 0..size {
                if (i + j) % 2 == 0 {
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

    pub fn stabilizer_measurement(&self, circuit: &mut QuantumCircuit) -> Result<(), JsValue> {
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

    fn get_data_neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if i > 0 { neighbors.push((i-1, j)); }
        if i < self.size-1 { neighbors.push((i+1, j)); }
        if j > 0 { neighbors.push((i, j-1)); }
        if j < self.size-1 { neighbors.push((i, j+1)); }
        neighbors.into_iter()
            .filter(|&(x, y)| self.data_qubits.contains(&(x, y)))
            .collect()
    }

    fn qubit_index(&self, i: usize, j: usize) -> usize {
        i * self.size + j
    }
}
