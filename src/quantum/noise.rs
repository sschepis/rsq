use wasm_bindgen::prelude::*;
use super::matrix::ComplexMatrix;
use super::complex::Complex;
use rand::Rng;

#[wasm_bindgen]
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

    pub fn apply_noise(&self, state: &mut ComplexMatrix) -> Result<(), JsValue> {
        let mut rng = rand::thread_rng();
        
        // Depolarizing noise
        if rng.gen::<f64>() < self.depolarizing_prob {
            let pauli = rng.gen_range(0..3);
            match pauli {
                0 => { let _ = state.multiply(&ComplexMatrix::pauli_x())?; }
                1 => { let _ = state.multiply(&ComplexMatrix::pauli_y())?; }
                2 => { let _ = state.multiply(&ComplexMatrix::pauli_z())?; }
                _ => unreachable!(),
            }
        }
        
        // Bit flip noise
        if rng.gen::<f64>() < self.bit_flip_prob {
            let _ = state.multiply(&ComplexMatrix::pauli_x())?;
        }
        
        // Phase flip noise
        if rng.gen::<f64>() < self.phase_flip_prob {
            let _ = state.multiply(&ComplexMatrix::pauli_z())?;
        }
        
        // Amplitude damping
        if rng.gen::<f64>() < self.amplitude_damping_prob {
            let gamma = self.amplitude_damping_prob;
            let mut damping = ComplexMatrix::new(2, 2);
            damping.set(0, 0, &Complex::new(1.0, 0.0));
            damping.set(1, 1, &Complex::new((1.0 - gamma).sqrt(), 0.0));
            let _ = state.multiply(&damping)?;
        }
        
        Ok(())
    }
}

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
        // Maximum likelihood estimation
        let mut rho = ComplexMatrix::new(2, 2);
        rho.set(0, 0, &Complex::new(0.5, 0.0));
        rho.set(1, 1, &Complex::new(0.5, 0.0));
        
        // Iterative reconstruction
        for _ in 0..100 {
            let mut r = ComplexMatrix::new(2, 2);
            
            for (measurement, &result) in self.measurements.iter().zip(self.results.iter()) {
                let trace = measurement.multiply(&rho)?.trace();
                let prob = trace.real;
                let correction = (result - prob) / prob;
                r = r.add(&measurement.scalar_multiply(correction))?;
            }
            
            rho = r.multiply(&rho)?.multiply(&r)?;
            let trace = rho.trace();
            rho = rho.scalar_multiply(1.0 / trace.real);
        }
        
        Ok(rho)
    }
}
