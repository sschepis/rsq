use super::super::*;
use super::super::hamiltonian::*;
use super::super::noise::*;
use super::super::tomography::*;

#[test]
fn test_state_evolution() {
    let mut state = QuantumState::new(2, BasisType::Computational);
    let hamiltonian = Hamiltonian::from_terms(vec![
        HamiltonianTerm::PauliX,
        HamiltonianTerm::PauliZ
    ]);
    
    state.evolve(1.0, &hamiltonian);
    assert!(state.amplitudes[0].norm() > 0.0);
}

#[test]
fn test_noise_application() {
    let mut state = QuantumState::new(2, BasisType::Computational);
    let noise_model = NoiseModel::new(NoiseType::Depolarizing(0.1));
    
    state.apply_noise(&noise_model);
    assert!(state.amplitudes[0].norm() > 0.0);
}

#[test]
fn test_state_reconstruction() {
    let mut state = QuantumState::new(2, BasisType::Computational);
    let reconstruction = StateReconstruction::new();
    
    state.metadata.reconstruction_data = Some(reconstruction);
    let result = state.reconstruct_state();
    
    assert!(result.is_ok());
}

#[test]
fn test_reconstruction_error() {
    let state = QuantumState::new(2, BasisType::Computational);
    let result = state.reconstruct_state();
    
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "State reconstruction failed"
    );
}
