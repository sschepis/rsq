use crate::quantum::core::complex::Complex;
use crate::quantum::core::matrix::ComplexMatrix;

pub fn grover_iteration(state: &mut ComplexMatrix, oracle: &ComplexMatrix) -> Result<(), &'static str> {
    // Apply oracle
    let result = oracle.multiply(state)?;
    *state = result;

    // Apply diffusion operator
    let mut diffusion = ComplexMatrix::new(state.rows(), state.rows());
    let n = state.rows();
    let factor = 2.0 / (n as f64);

    for i in 0..n {
        for j in 0..n {
            diffusion.set(i, j, Complex::new(factor, 0.0));
        }
        // Subtract identity
        diffusion.set(i, i, Complex::new(factor - 1.0, 0.0));
    }

    let result = diffusion.multiply(state)?;
    *state = result;
    Ok(())
}

pub fn quantum_fourier_transform(state: &mut ComplexMatrix) -> Result<(), &'static str> {
    let n = state.rows();
    let mut qft = ComplexMatrix::new(n, n);
    let factor = 1.0 / (n as f64).sqrt();

    for i in 0..n {
        for j in 0..n {
            let phase = 2.0 * std::f64::consts::PI * (i as f64) * (j as f64) / (n as f64);
            qft.set(i, j, Complex::new(
                factor * phase.cos(),
                factor * phase.sin()
            ));
        }
    }

    let result = qft.multiply(state)?;
    *state = result;
    Ok(())
}

pub fn phase_estimation(unitary: &ComplexMatrix, state: &mut ComplexMatrix, precision: usize) -> Result<Vec<f64>, &'static str> {
    let n = state.rows();
    let mut phases = Vec::with_capacity(n);

    // Initialize control qubits
    let mut control_state = ComplexMatrix::new(2usize.pow(precision as u32), 1);
    control_state.set(0, 0, Complex::new(1.0, 0.0));

    // Apply QFT
    quantum_fourier_transform(&mut control_state)?;

    // Apply controlled unitary operations
    for i in 0..precision {
        let power = 2u32.pow(i as u32);
        let mut controlled_u = ComplexMatrix::identity(n * 2);
        
        for j in 0..n {
            let phase = 2.0 * std::f64::consts::PI * (j as f64) * (power as f64) / (n as f64);
            controlled_u.set(j, j, Complex::new(phase.cos(), phase.sin()));
        }

        let result = controlled_u.multiply(state)?;
        *state = result;
    }

    // Inverse QFT
    let mut qft = ComplexMatrix::new(n, n);
    let factor = 1.0 / (n as f64).sqrt();
    
    for i in 0..n {
        for j in 0..n {
            let phase = -2.0 * std::f64::consts::PI * (i as f64) * (j as f64) / (n as f64);
            qft.set(i, j, Complex::new(
                factor * phase.cos(),
                factor * phase.sin()
            ));
        }
    }

    let result = qft.multiply(state)?;
    *state = result;

    // Extract phases
    for i in 0..n {
        let prob = state.get(i, 0).norm_sqr();
        if prob > 1e-10 {
            phases.push(i as f64 / n as f64);
        }
    }

    Ok(phases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grover_iteration() {
        let mut state = ComplexMatrix::new(4, 1);
        state.set(0, 0, Complex::new(0.5, 0.0));
        state.set(1, 0, Complex::new(0.5, 0.0));
        state.set(2, 0, Complex::new(0.5, 0.0));
        state.set(3, 0, Complex::new(0.5, 0.0));

        let mut oracle = ComplexMatrix::new(4, 4);
        oracle.set(0, 0, Complex::new(1.0, 0.0));
        oracle.set(1, 1, Complex::new(1.0, 0.0));
        oracle.set(2, 2, Complex::new(1.0, 0.0));
        oracle.set(3, 3, Complex::new(-1.0, 0.0));

        assert!(grover_iteration(&mut state, &oracle).is_ok());
    }

    #[test]
    fn test_qft() {
        let mut state = ComplexMatrix::new(4, 1);
        state.set(0, 0, Complex::new(1.0, 0.0));

        assert!(quantum_fourier_transform(&mut state).is_ok());
    }
}
