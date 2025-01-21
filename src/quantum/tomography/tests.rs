use super::*;
use crate::quantum::core::matrix::ComplexMatrix;
use crate::quantum::core::complex::Complex;
use std::f64::EPSILON;

#[test]
fn test_quantum_tomography_creation() {
    let tomography = QuantumTomography::new();
    assert_eq!(tomography.measurements.len(), 0);
    assert_eq!(tomography.results.len(), 0);
}

#[test]
fn test_quantum_tomography_add_measurement() {
    let mut tomography = QuantumTomography::new();
    
    // Add measurements in different bases
    tomography.add_measurement("X", 1.0);
    tomography.add_measurement("Y", -1.0);
    tomography.add_measurement("Z", 0.0);
    
    assert_eq!(tomography.measurements.len(), 3);
    assert_eq!(tomography.results.len(), 3);
    assert_eq!(tomography.results[0], 1.0);
    assert_eq!(tomography.results[1], -1.0);
    assert_eq!(tomography.results[2], 0.0);
}

#[test]
fn test_quantum_tomography_invalid_basis() {
    let mut tomography = QuantumTomography::new();
    
    // Invalid basis should default to identity
    tomography.add_measurement("invalid", 1.0);
    
    assert_eq!(tomography.measurements.len(), 1);
    assert_eq!(tomography.results.len(), 1);
    
    // Check if the measurement matrix is identity
    let measurement = &tomography.measurements[0];
    let m00 = measurement.get(0, 0);
    let m11 = measurement.get(1, 1);
    let m01 = measurement.get(0, 1);
    let m10 = measurement.get(1, 0);
    
    assert!((m00.real - 1.0).abs() < EPSILON);
    assert!((m11.real - 1.0).abs() < EPSILON);
    assert!(m01.real.abs() < EPSILON);
    assert!(m10.real.abs() < EPSILON);
}

#[test]
fn test_state_reconstruction() -> Result<(), JsValue> {
    let mut tomography = QuantumTomography::new();
    
    // Add measurements for a simple state (|0⟩)
    tomography.add_measurement("Z", 1.0);  // Fully polarized in Z
    tomography.add_measurement("X", 0.0);  // Random in X
    tomography.add_measurement("Y", 0.0);  // Random in Y
    
    let rho = tomography.reconstruct_state()?;
    
    // Check if reconstructed state is close to |0⟩⟨0|
    let rho_00 = rho.get(0, 0);
    let rho_11 = rho.get(1, 1);
    
    assert!((rho_00.real - 1.0).abs() < 0.1);  // Allowing some numerical error
    assert!(rho_11.real.abs() < 0.1);
    
    Ok(())
}
