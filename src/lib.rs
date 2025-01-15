pub mod quantum;
pub mod mining;

// Register resonance module
pub use quantum::resonance::{ResonanceFunction, QuantumResonanceOptimizer};

use wasm_bindgen::prelude::*;
use quantum::{Complex, ComplexMatrix};

// Re-export main types for convenience
pub use quantum::{
    QuantumCircuit,
    ErrorCorrection,
    SurfaceCode,
    QuantumAlgorithms,
    NoiseModel,
    QuantumTomography,
    resonance,
};

// Re-export mining types
pub use mining::{
    MiningOptions,
    NonceResult,
    mine_async,
};

// Utility functions for quantum states
#[wasm_bindgen]
pub fn create_bell_state() -> ComplexMatrix {
    let mut state = ComplexMatrix::new(4, 1);
    let coeff = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
    state.set(0, 0, &coeff);
    state.set(3, 0, &coeff);
    state
}

#[wasm_bindgen]
pub fn create_ghz_state(n_qubits: usize) -> ComplexMatrix {
    let size = 1 << n_qubits;
    let mut state = ComplexMatrix::new(size, 1);
    let coeff = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
    state.set(0, 0, &coeff);
    state.set(size-1, 0, &coeff);
    state
}

// Re-export quantum gate constructors
pub use quantum::{
    controlled_rotation,
    toffoli_phase,
};
