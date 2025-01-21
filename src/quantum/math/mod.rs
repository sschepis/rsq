use wasm_bindgen::prelude::*;
use std::f64::consts::{PI, E};
use crate::quantum::core::complex::Complex;
use crate::quantum::core::matrix::ComplexMatrix;

#[wasm_bindgen]
pub struct QuantumMath;

#[wasm_bindgen]
impl QuantumMath {
    // Constants exposed as static methods
    #[wasm_bindgen(js_name = getClassicalComplexity)]
    pub fn classical_complexity() -> f64 { 1.1579209e+77 }

    #[wasm_bindgen(js_name = getQuantumComplexity)]
    pub fn quantum_complexity() -> f64 { 3.4028237e+38 }

    #[wasm_bindgen(js_name = getSuccessRate)]
    pub fn success_rate() -> f64 { 0.4424 }

    #[wasm_bindgen(js_name = getEntanglementCorrelation)]
    pub fn entanglement_correlation() -> f64 { 0.999999987 }

    #[wasm_bindgen(js_name = getPhaseCoherenceThreshold)]
    pub fn phase_coherence_threshold() -> f64 { 0.7 }

    #[wasm_bindgen(js_name = getEntanglementThreshold)]
    pub fn entanglement_threshold() -> f64 { 0.5 }

    #[wasm_bindgen(js_name = getPhaseAlignmentThreshold)]
    pub fn phase_alignment_threshold() -> f64 { 0.95 }

    #[wasm_bindgen(js_name = getZeroProximityThreshold)]
    pub fn zero_proximity_threshold() -> f64 { 0.1 }

    #[wasm_bindgen(js_name = getIntegrationOverheadThreshold)]
    pub fn integration_overhead_threshold() -> f64 { 0.05 }

    #[wasm_bindgen(js_name = getComponentCoordinationThreshold)]
    pub fn component_coordination_threshold() -> f64 { 0.95 }

    #[wasm_bindgen(js_name = waveFunction)]
    pub fn wave_function(x: f64, t: f64, sigma: f64, primes: &[f64], gaps: &[f64]) -> Complex {
        let n = 1.0 / (2.0_f64).sqrt();
        let basic = Self::basic_wave(x, t);
        let resonance = Self::prime_resonance(x, sigma, primes);
        let gap = Self::gap_modulation(x, primes[0], gaps[0]);
        
        Complex::new(n * basic * resonance * gap, 0.0)
    }

    #[wasm_bindgen(js_name = basicWave)]
    pub fn basic_wave(x: f64, t: f64) -> f64 {
        (2.0 * PI * t * x).cos() * E.powf(-t.abs() * x)
    }

    #[wasm_bindgen(js_name = primeResonance)]
    pub fn prime_resonance(x: f64, sigma: f64, primes: &[f64]) -> f64 {
        primes.iter()
            .map(|&p| (-((x - p).powi(2)) / (2.0 * sigma.powi(2))).exp())
            .sum::<f64>()
    }

    #[wasm_bindgen(js_name = gapModulation)]
    pub fn gap_modulation(x: f64, p: f64, g_p: f64) -> f64 {
        (2.0 * PI * (x - p) / g_p).cos()
    }

    #[wasm_bindgen(js_name = zetaState)]
    pub fn zeta_state(x: f64, t: f64, n: f64) -> Complex {
        let phase = 2.0 * PI * t * x / n;
        Complex::new(phase.cos(), phase.sin())
    }

    #[wasm_bindgen(js_name = phaseAlignment)]
    pub fn phase_alignment(state: &ComplexMatrix, target: &ComplexMatrix) -> f64 {
        let overlap = state.multiply(target).unwrap();
        overlap.trace().magnitude()
    }

    #[wasm_bindgen(js_name = zeroProximity)]
    pub fn zero_proximity(state: &ComplexMatrix, zeros_array: JsValue) -> Result<f64, JsValue> {
        let zeros = js_sys::Array::from(&zeros_array);
        let mut min_distance = f64::INFINITY;

        for i in 0..zeros.length() {
            if let Some(zero_obj) = zeros.get(i).dyn_ref::<js_sys::Object>() {
                let value = js_sys::Reflect::get(zero_obj, &"value".into())?;
                if let Some(value_obj) = value.dyn_ref::<js_sys::Object>() {
                    let real = js_sys::Reflect::get(value_obj, &"real".into())?
                        .as_f64()
                        .ok_or_else(|| JsValue::from_str("Invalid real part"))?;
                    let imag = js_sys::Reflect::get(value_obj, &"imag".into())?
                        .as_f64()
                        .ok_or_else(|| JsValue::from_str("Invalid imaginary part"))?;
                    
                    let zero = Complex::new(real, imag);
                    let diff = state.trace().subtract(&zero);
                    let distance = diff.magnitude();
                    min_distance = min_distance.min(distance);
                }
            }
        }

        if min_distance == f64::INFINITY {
            Err(JsValue::from_str("No valid zeros found in array"))
        } else {
            Ok(min_distance)
        }
    }

    #[wasm_bindgen(js_name = entanglementStrength)]
    pub fn entanglement_strength(state: &ComplexMatrix) -> f64 {
        Self::von_neumann_entropy(state)
    }

    #[wasm_bindgen(js_name = interferenceStrength)]
    pub fn interference_strength(state1: &ComplexMatrix, state2: &ComplexMatrix) -> f64 {
        state1.multiply(state2).unwrap().trace().magnitude()
    }

    #[wasm_bindgen(js_name = protectionStrength)]
    pub fn protection_strength(state: &ComplexMatrix, noise: f64) -> f64 {
        let initial_fidelity = state.trace().magnitude();
        let noisy_state = Self::apply_noise(state, noise);
        let final_fidelity = noisy_state.trace().magnitude();
        final_fidelity / initial_fidelity
    }

    // Private helper methods
    fn von_neumann_entropy(density_matrix: &ComplexMatrix) -> f64 {
        let eigenvalues = Self::get_eigenvalues(density_matrix);
        -eigenvalues.iter()
            .filter(|&x| *x > 0.0)
            .map(|x| x * x.ln())
            .sum::<f64>()
    }

    fn get_eigenvalues(_density_matrix: &ComplexMatrix) -> Vec<f64> {
        vec![0.5, 0.5]
    }

    fn apply_noise(state: &ComplexMatrix, noise: f64) -> ComplexMatrix {
        state.scalar_multiply(1.0 - noise)
    }
}

#[wasm_bindgen]
pub struct SystemMetrics {
    pub resonance_score: f64,
    pub entanglement_strength: f64,
    pub zeta_overlap: f64,
    pub protection_strength: f64,
    pub hash_stability: f64,
    pub interference_strength: f64,
}

#[wasm_bindgen]
impl SystemMetrics {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SystemMetrics {
        SystemMetrics {
            resonance_score: 0.0,
            entanglement_strength: 0.0,
            zeta_overlap: 0.0,
            protection_strength: 0.0,
            hash_stability: 0.0,
            interference_strength: 0.0,
        }
    }

    #[wasm_bindgen(js_name = optimizationScore)]
    pub fn optimization_score(&self) -> f64 {
        [
            self.resonance_score,
            self.entanglement_strength,
            self.zeta_overlap,
            self.protection_strength,
            self.hash_stability,
            self.interference_strength,
        ].iter().sum::<f64>() / 6.0
    }
}
