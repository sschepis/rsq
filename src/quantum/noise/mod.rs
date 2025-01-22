use crate::quantum::core::complex::Complex;
use crate::quantum::core::matrix::ComplexMatrix;

pub enum NoiseType {
    Depolarizing,
    BitFlip,
    PhaseFlip,
    AmplitudeDamping,
    Custom(Box<dyn Fn(f64) -> ComplexMatrix>),
}

pub struct NoiseChannel {
    noise_type: NoiseType,
    strength: f64,
}

impl NoiseChannel {
    pub fn new(noise_type: NoiseType, strength: f64) -> Self {
        NoiseChannel {
            noise_type,
            strength,
        }
    }

    pub fn apply(&self, state: &mut ComplexMatrix) -> Result<(), &'static str> {
        let noise_matrix = match self.noise_type {
            NoiseType::Depolarizing => self.depolarizing_channel(self.strength),
            NoiseType::BitFlip => self.bit_flip_channel(self.strength),
            NoiseType::PhaseFlip => self.phase_flip_channel(self.strength),
            NoiseType::AmplitudeDamping => self.amplitude_damping_channel(self.strength),
            NoiseType::Custom(ref f) => f(self.strength),
        };

        let result = noise_matrix.multiply(state)?;
        *state = result;
        Ok(())
    }

    fn depolarizing_channel(&self, p: f64) -> ComplexMatrix {
        let mut matrix = ComplexMatrix::new(2, 2);
        matrix.set(0, 0, Complex::new(1.0 - p, 0.0));
        matrix.set(1, 1, Complex::new(1.0 - p, 0.0));
        matrix.set(0, 1, Complex::new(p/3.0, 0.0));
        matrix.set(1, 0, Complex::new(p/3.0, 0.0));
        matrix
    }

    fn bit_flip_channel(&self, p: f64) -> ComplexMatrix {
        let mut matrix = ComplexMatrix::new(2, 2);
        matrix.set(0, 0, Complex::new(1.0 - p, 0.0));
        matrix.set(1, 1, Complex::new(1.0 - p, 0.0));
        matrix.set(0, 1, Complex::new(p, 0.0));
        matrix.set(1, 0, Complex::new(p, 0.0));
        matrix
    }

    fn phase_flip_channel(&self, p: f64) -> ComplexMatrix {
        let mut matrix = ComplexMatrix::new(2, 2);
        matrix.set(0, 0, Complex::new(1.0 - p, 0.0));
        matrix.set(1, 1, Complex::new(-(1.0 - p), 0.0));
        matrix.set(0, 1, Complex::new(0.0, p));
        matrix.set(1, 0, Complex::new(0.0, -p));
        matrix
    }

    fn amplitude_damping_channel(&self, gamma: f64) -> ComplexMatrix {
        let mut matrix = ComplexMatrix::new(2, 2);
        matrix.set(0, 0, Complex::new(1.0, 0.0));
        matrix.set(1, 1, Complex::new((1.0 - gamma).sqrt(), 0.0));
        matrix.set(0, 1, Complex::new(gamma.sqrt(), 0.0));
        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depolarizing_channel() {
        let noise = NoiseChannel::new(NoiseType::Depolarizing, 0.1);
        let mut state = ComplexMatrix::new(2, 2);
        state.set(0, 0, Complex::new(1.0, 0.0));
        assert!(noise.apply(&mut state).is_ok());
    }

    #[test]
    fn test_bit_flip_channel() {
        let noise = NoiseChannel::new(NoiseType::BitFlip, 0.1);
        let mut state = ComplexMatrix::new(2, 2);
        state.set(0, 0, Complex::new(1.0, 0.0));
        assert!(noise.apply(&mut state).is_ok());
    }

    #[test]
    fn test_phase_flip_channel() {
        let noise = NoiseChannel::new(NoiseType::PhaseFlip, 0.1);
        let mut state = ComplexMatrix::new(2, 2);
        state.set(0, 0, Complex::new(1.0, 0.0));
        assert!(noise.apply(&mut state).is_ok());
    }

    #[test]
    fn test_amplitude_damping_channel() {
        let noise = NoiseChannel::new(NoiseType::AmplitudeDamping, 0.1);
        let mut state = ComplexMatrix::new(2, 2);
        state.set(0, 0, Complex::new(1.0, 0.0));
        assert!(noise.apply(&mut state).is_ok());
    }
}
