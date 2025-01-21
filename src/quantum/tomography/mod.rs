use wasm_bindgen::prelude::*;
use crate::quantum::core::matrix::ComplexMatrix;
use crate::quantum::core::complex::Complex;

#[cfg(test)]
mod tests;

#[wasm_bindgen]
pub struct QuantumTomography {
    measurements: Vec<ComplexMatrix>,
    results: Vec<f64>,
}

#[wasm_bindgen]
impl QuantumTomography {
    #[wasm_bindgen(constructor)]
    pub fn new() -> QuantumTomography {
        QuantumTomography {
            measurements: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn add_measurement(&mut self, basis: &str, result: f64) {
        let measurement = match basis {
            "X" => ComplexMatrix::pauli_x(),
            "Y" => ComplexMatrix::pauli_y(),
            "Z" => ComplexMatrix::pauli_z(),
            _ => ComplexMatrix::identity(2),
        };
        
        self.measurements.push(measurement);
        self.results.push(result);
    }

    pub fn reconstruct_state(&self) -> Result<ComplexMatrix, JsValue> {
        // Initialize density matrix as maximally mixed state
        let mut rho = ComplexMatrix::new(2, 2);
        rho.set(0, 0, &Complex::new(0.5, 0.0));
        rho.set(1, 1, &Complex::new(0.5, 0.0));
        
        let learning_rate = 0.1;
        let mut prev_fidelity = 0.0;
        
        // Iterative reconstruction
        for iter in 0..5000 {
            let mut gradient = ComplexMatrix::new(2, 2);
            
            // Compute gradient from all measurements
            for (measurement, &result) in self.measurements.iter().zip(self.results.iter()) {
                let expect = measurement.multiply(&rho)?.trace().real;
                let diff = result - expect;
                gradient = gradient.add(&measurement.scalar_multiply(diff))?;
            }
            
            // Update step with momentum
            let step_size = learning_rate / (1.0 + (iter as f64) * 0.01);
            let update = gradient.scalar_multiply(step_size);
            rho = rho.add(&update)?;
            
            // Ensure hermiticity and positivity
            let rho_dag = rho.adjoint();
            rho = rho.add(&rho_dag)?.scalar_multiply(0.5);
            
            // Normalize
            let trace = rho.trace().real;
            if trace.abs() > 1e-10 {
                rho = rho.scalar_multiply(1.0 / trace);
            }
            
            // Check convergence
            let fidelity = rho.get(0, 0).real;
            if (fidelity - prev_fidelity).abs() < 1e-6 && iter > 100 {
                break;
            }
            prev_fidelity = fidelity;
        }
        
        Ok(rho)
    }
}
