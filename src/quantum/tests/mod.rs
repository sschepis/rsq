use super::*;
use proptest::prelude::*;

mod state_reconstruction_tests {
    use super::*;

    #[test]
    fn test_reconstruct_state_basic() {
        let measurements = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 1.0),
            Complex::new(1.0, 1.0).normalize()
        ];
        
        let noise = NoiseModel {
            depolarizing_rate: 0.01,
            dephasing_rate: 0.01,
            amplitude_damping_rate: 0.01
        };

        let result = reconstruct_state(&measurements, noise);
        assert!(result.is_ok());
        let density_matrix = result.unwrap();
        
        // Verify trace is 1
        assert!((density_matrix.trace().re - 1.0).abs() < 1e-6);
        
        // Verify positive semi-definite
        assert!(density_matrix.is_positive_semi_definite());
    }

    proptest! {
        #[test]
        fn test_reconstruct_state_properties(measurements in prop::collection::vec(complex_number(), 3..100)) {
            let noise = NoiseModel {
                depolarizing_rate: 0.01,
                dephasing_rate: 0.01,
                amplitude_damping_rate: 0.01
            };

            let result = reconstruct_state(&measurements, noise);
            prop_assert!(result.is_ok());
            let density_matrix = result.unwrap();
            
            // Verify trace is 1
            prop_assert!((density_matrix.trace().re - 1.0).abs() < 1e-6);
            
            // Verify positive semi-definite
            prop_assert!(density_matrix.is_positive_semi_definite());
        }
    }
}

mod hamiltonian_evolution_tests {
    use super::*;

    #[test]
    fn test_unitary_evolution() {
        let state = ComplexMatrix::identity(2);
        let hamiltonian = ComplexMatrix::from_vec(2, 2, vec![
            Complex::new(1.0, 0.0), Complex::new(0.0, -1.0),
            Complex::new(0.0, 1.0), Complex::new(1.0, 0.0)
        ]);
        
        let evolved = evolve_state(&state, &hamiltonian, 1.0);
        
        // Verify unitarity
        let product = evolved * evolved.adjoint();
        assert!(product.is_identity(1e-6));
    }

    #[test]
    fn test_energy_conservation() {
        let state = ComplexMatrix::from_vec(2, 1, vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0)
        ]);
        
        let hamiltonian = ComplexMatrix::from_vec(2, 2, vec![
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(2.0, 0.0)
        ]);
        
        let initial_energy = (state.adjoint() * hamiltonian * state).trace().re;
        let evolved = evolve_state(&state, &hamiltonian, 1.0);
        let final_energy = (evolved.adjoint() * hamiltonian * evolved).trace().re;
        
        assert!((initial_energy - final_energy).abs() < 1e-6);
    }
}

mod noise_modeling_tests {
    use super::*;

    #[test]
    fn test_depolarizing_channel() {
        let state = ComplexMatrix::identity(2);
        let noise = NoiseModel {
            depolarizing_rate: 0.1,
            dephasing_rate: 0.0,
            amplitude_damping_rate: 0.0
        };
        
        let result = apply_noise(&state, noise);
        assert!(result.is_ok());
        let noisy_state = result.unwrap();
        
        // Verify trace preservation
        assert!((noisy_state.trace().re - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_error_correction() {
        let state = ComplexMatrix::from_vec(2, 1, vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0)
        ]);
        
        let noise = NoiseModel {
            depolarizing_rate: 0.1,
            dephasing_rate: 0.1,
            amplitude_damping_rate: 0.1
        };
        
        let noisy_state = apply_noise(&state, noise).unwrap();
        let corrected_state = error_correction(&noisy_state);
        
        // Verify fidelity improvement
        let initial_fidelity = state.fidelity(&noisy_state);
        let final_fidelity = state.fidelity(&corrected_state);
        assert!(final_fidelity > initial_fidelity);
    }
}

// Helper functions for property-based testing
fn complex_number() -> impl Strategy<Value = Complex> {
    (-100.0..100.0).prop_flat_map(|re| 
        (-100.0..100.0).prop_map(move |im| 
            Complex::new(re, im)
        )
    )
}
