use wasm_bindgen::prelude::*;
use crate::quantum::core::matrix::ComplexMatrix;
use crate::quantum::core::complex::Complex;

#[wasm_bindgen]
#[derive(Clone)]
pub struct QuantumCircuit {
    pub(crate) num_qubits: usize,
    pub(crate) gates: Vec<(String, Vec<usize>, Option<f64>)>,
    pub(crate) state: ComplexMatrix,
}

#[wasm_bindgen]
impl QuantumCircuit {
    #[wasm_bindgen(constructor)]
    pub fn new(num_qubits: usize) -> QuantumCircuit {
        let size = 1 << num_qubits;
        let mut state = ComplexMatrix::new(size, 1);
        state.set(0, 0, &Complex::new(1.0, 0.0));
        
        QuantumCircuit {
            num_qubits,
            gates: Vec::new(),
            state,
        }
    }

    pub fn add_gate(&mut self, gate_name: &str, qubits: Vec<usize>, parameter: Option<f64>) {
        self.gates.push((gate_name.to_string(), qubits, parameter));
    }

    pub fn add_controlled_u(&mut self, control: usize, target: usize, unitary: &ComplexMatrix) -> Result<(), JsValue> {
        let size = 1 << self.num_qubits;
        let mut cu = ComplexMatrix::identity(size);

        let control_mask = 1 << control;

        for i in 0..size {
            if (i & control_mask) != 0 {
                for j in 0..2 {
                    for k in 0..2 {
                        let mut new_state = i;
                        new_state &= !(1 << target);
                        new_state |= j << target;
                        
                        let mut final_state = i;
                        final_state &= !(1 << target);
                        final_state |= k << target;
                        
                        cu.set(final_state, new_state, &unitary.get(k, j));
                    }
                }
            }
        }

        self.state = cu.multiply(&self.state)?;
        Ok(())
    }

    pub fn simulate(&mut self) -> Result<(), JsValue> {
        for (gate_name, qubits, parameter) in self.gates.clone() {
            match gate_name.as_str() {
                "H" => self.apply_hadamard(qubits[0])?,
                "X" => self.apply_pauli_x(qubits[0])?,
                "Y" => self.apply_pauli_y(qubits[0])?,
                "Z" => self.apply_pauli_z(qubits[0])?,
                "CNOT" => self.apply_cnot(qubits[0], qubits[1])?,
                "P" => self.apply_phase(qubits[0], parameter.unwrap_or(0.0))?,
                "T" => self.apply_t(qubits[0])?,
                "SWAP" => self.apply_swap(qubits[0], qubits[1])?,
                _ => return Err(JsValue::from_str("Unknown gate")),
            }
        }
        Ok(())
    }

    pub fn encode_bit_flip(&mut self, qubit: usize) -> Result<(), JsValue> {
        if qubit >= self.num_qubits - 2 {
            return Err(JsValue::from_str("Insufficient qubits for encoding"));
        }

        self.apply_cnot(qubit, qubit + 1)?;
        self.apply_cnot(qubit, qubit + 2)?;
        Ok(())
    }

    pub fn encode_steane(&mut self, qubit: usize) -> Result<(), JsValue> {
        if qubit >= self.num_qubits - 6 {
            return Err(JsValue::from_str("Insufficient qubits for Steane code"));
        }

        for i in 1..7 {
            self.apply_hadamard(qubit + i)?;
        }

        let cnot_pattern = vec![
            (0, 1), (0, 2), (0, 3), (1, 4), (2, 5), (3, 6),
            (0, 4), (0, 5), (0, 6), (1, 5), (1, 6), (2, 6)
        ];

        for (control, target) in cnot_pattern {
            self.apply_cnot(qubit + control, qubit + target)?;
        }
        Ok(())
    }

    pub fn detect_errors(&self) -> Result<JsValue, JsValue> {
        let errors = js_sys::Array::new();
        for i in 0..self.num_qubits {
            let measurement = self.state.measure_qubit(i, self.num_qubits)?;
            let proj1 = js_sys::Reflect::get(&measurement, &"1".into())?;
            let prob = proj1.as_f64().unwrap_or(0.0);
            errors.push(&JsValue::from_bool(prob > 0.5));
        }
        Ok(errors.into())
    }

    pub fn to_density_matrix(&self) -> ComplexMatrix {
        let state_conj = self.state.adjoint();
        self.state.multiply(&state_conj).unwrap()
    }

    pub fn apply_hadamard(&mut self, qubit: usize) -> Result<(), JsValue> {
        let sqrt2_inv = 1.0 / 2.0_f64.sqrt();
        let mut h = ComplexMatrix::new(2, 2);
        h.set(0, 0, &Complex::new(sqrt2_inv, 0.0));
        h.set(0, 1, &Complex::new(sqrt2_inv, 0.0));
        h.set(1, 0, &Complex::new(sqrt2_inv, 0.0));
        h.set(1, 1, &Complex::new(-sqrt2_inv, 0.0));
        
        self.apply_single_qubit_gate(qubit, &h)
    }

    pub fn apply_pauli_x(&mut self, qubit: usize) -> Result<(), JsValue> {
        self.apply_single_qubit_gate(qubit, &ComplexMatrix::pauli_x())
    }

    pub fn apply_pauli_y(&mut self, qubit: usize) -> Result<(), JsValue> {
        self.apply_single_qubit_gate(qubit, &ComplexMatrix::pauli_y())
    }

    pub fn apply_pauli_z(&mut self, qubit: usize) -> Result<(), JsValue> {
        self.apply_single_qubit_gate(qubit, &ComplexMatrix::pauli_z())
    }

    pub fn apply_phase(&mut self, qubit: usize, phi: f64) -> Result<(), JsValue> {
        let mut p = ComplexMatrix::new(2, 2);
        p.set(0, 0, &Complex::new(1.0, 0.0));
        p.set(1, 1, &Complex::new(phi.cos(), phi.sin()));
        self.apply_single_qubit_gate(qubit, &p)
    }

    pub fn apply_t(&mut self, qubit: usize) -> Result<(), JsValue> {
        let mut t = ComplexMatrix::new(2, 2);
        t.set(0, 0, &Complex::new(1.0, 0.0));
        t.set(1, 1, &Complex::new(1.0/2.0_f64.sqrt(), 1.0/2.0_f64.sqrt()));
        self.apply_single_qubit_gate(qubit, &t)
    }

    pub fn apply_cnot(&mut self, control: usize, target: usize) -> Result<(), JsValue> {
        self.apply_controlled_gate(control, target, &ComplexMatrix::pauli_x())
    }

    pub fn apply_swap(&mut self, qubit1: usize, qubit2: usize) -> Result<(), JsValue> {
        let size = 1 << self.num_qubits;
        let mut full_swap = ComplexMatrix::identity(size);
        
        for i in 0..size {
            let bit1 = (i >> qubit1) & 1;
            let bit2 = (i >> qubit2) & 1;
            if bit1 != bit2 {
                let j = i ^ (1 << qubit1) ^ (1 << qubit2);
                full_swap.set(i, j, &Complex::new(1.0, 0.0));
                full_swap.set(j, i, &Complex::new(1.0, 0.0));
                full_swap.set(i, i, &Complex::new(0.0, 0.0));
                full_swap.set(j, j, &Complex::new(0.0, 0.0));
            }
        }
        
        self.state = full_swap.multiply(&self.state)?;
        Ok(())
    }

    fn apply_single_qubit_gate(&mut self, qubit: usize, gate: &ComplexMatrix) -> Result<(), JsValue> {
        let size = 1 << self.num_qubits;
        let mut full_gate = ComplexMatrix::identity(size);
        
        for i in 0..size {
            let bit = (i >> qubit) & 1;
            for j in 0..2 {
                let new_i = (i & !(1 << qubit)) | (j << qubit);
                full_gate.set(new_i, i, &gate.get(j, bit));
            }
        }
        
        self.state = full_gate.multiply(&self.state)?;
        Ok(())
    }

    fn apply_controlled_gate(&mut self, control: usize, target: usize, gate: &ComplexMatrix) -> Result<(), JsValue> {
        let size = 1 << self.num_qubits;
        let mut full_gate = ComplexMatrix::identity(size);
        
        for i in 0..size {
            if (i & (1 << control)) != 0 {
                let bit = (i >> target) & 1;
                for j in 0..2 {
                    let new_i = (i & !(1 << target)) | (j << target);
                    full_gate.set(new_i, i, &gate.get(j, bit));
                }
            }
        }
        
        self.state = full_gate.multiply(&self.state)?;
        Ok(())
    }

    #[cfg(test)]
    pub fn get_num_gates(&self) -> usize {
        self.gates.len()
    }
}
