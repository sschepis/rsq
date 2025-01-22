use crate::quantum::core::complex::Complex;
use crate::quantum::core::matrix::ComplexMatrix;

pub struct MeasurementBasis {
    pub matrix: ComplexMatrix,
    pub name: String,
}

impl MeasurementBasis {
    pub fn new(matrix: ComplexMatrix, name: String) -> Self {
        MeasurementBasis { matrix, name }
    }
}

pub struct QuantumTomography {
    bases: Vec<MeasurementBasis>,
}

impl QuantumTomography {
    pub fn new() -> Self {
        QuantumTomography { bases: Vec::new() }
    }

    pub fn add_basis(&mut self, basis: MeasurementBasis) {
        self.bases.push(basis);
    }

    pub fn reconstruct_state(&self, measurements: &[f64]) -> Result<ComplexMatrix, &'static str> {
        if measurements.len() != self.bases.len() {
            return Err("Number of measurements must match number of bases");
        }

        let mut reconstructed = ComplexMatrix::new(2, 2);
        reconstructed.set(0, 0, Complex::new(1.0, 0.0));

        for (i, measurement) in measurements.iter().enumerate() {
            let projection = self.bases[i].matrix.multiply(&reconstructed)?;
            let prob = projection.get(0, 0).norm_sqr();
            
            if prob > 1e-10 {
                let factor = measurement / prob.sqrt();
                reconstructed = projection;
                for j in 0..reconstructed.rows() {
                    for k in 0..reconstructed.cols() {
                        let val = reconstructed.get(j, k);
                        reconstructed.set(j, k, val * Complex::new(factor, 0.0));
                    }
                }
            }
        }

        Ok(reconstructed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tomography() {
        let mut tomo = QuantumTomography::new();
        
        let mut basis = ComplexMatrix::new(2, 2);
        basis.set(0, 0, Complex::new(1.0, 0.0));
        basis.set(1, 1, Complex::new(1.0, 0.0));
        
        tomo.add_basis(MeasurementBasis::new(basis, "Z".to_string()));
        
        let measurements = vec![1.0];
        let result = tomo.reconstruct_state(&measurements);
        assert!(result.is_ok());
    }
}
