use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

#[wasm_bindgen]
impl Complex {
    #[wasm_bindgen(constructor)]
    pub fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }

    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    pub fn subtract(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }

    pub fn multiply(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }

    pub fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    pub fn phase(&self) -> f64 {
        self.imag.atan2(self.real)
    }

    pub fn exp(&self) -> Complex {
        let exp_real = self.real.exp();
        Complex {
            real: exp_real * self.imag.cos(),
            imag: exp_real * self.imag.sin(),
        }
    }

    pub fn norm(&self) -> f64 {
        self.magnitude()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    const EPSILON: f64 = 1e-10;

    fn assert_complex_eq(a: &Complex, b: &Complex) {
        assert!((a.real - b.real).abs() < EPSILON, "Real parts differ: {} != {}", a.real, b.real);
        assert!((a.imag - b.imag).abs() < EPSILON, "Imaginary parts differ: {} != {}", a.imag, b.imag);
    }

    #[test]
    fn test_new() {
        let z = Complex::new(3.0, 4.0);
        assert_eq!(z.real, 3.0);
        assert_eq!(z.imag, 4.0);
    }

    #[test]
    fn test_add() {
        let z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);
        let result = z1.add(&z2);
        assert_complex_eq(&result, &Complex::new(4.0, 6.0));
    }

    #[test]
    fn test_subtract() {
        let z1 = Complex::new(3.0, 4.0);
        let z2 = Complex::new(1.0, 2.0);
        let result = z1.subtract(&z2);
        assert_complex_eq(&result, &Complex::new(2.0, 2.0));
    }

    #[test]
    fn test_multiply() {
        let z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);
        let result = z1.multiply(&z2);
        assert_complex_eq(&result, &Complex::new(-5.0, 10.0));
    }

    #[test]
    fn test_conjugate() {
        let z = Complex::new(1.0, 2.0);
        let result = z.conjugate();
        assert_complex_eq(&result, &Complex::new(1.0, -2.0));
    }

    #[test]
    fn test_magnitude() {
        let z = Complex::new(3.0, 4.0);
        assert!((z.magnitude() - 5.0).abs() < EPSILON);
    }

    #[test]
    fn test_phase() {
        let z = Complex::new(1.0, 1.0);
        assert!((z.phase() - PI/4.0).abs() < EPSILON);

        let z = Complex::new(-1.0, 0.0);
        assert!((z.phase() - PI).abs() < EPSILON);
    }

    #[test]
    fn test_exp() {
        // Test e^(iπ) = -1
        let z = Complex::new(0.0, PI);
        let result = z.exp();
        assert_complex_eq(&result, &Complex::new(-1.0, 0.0));

        // Test e^0 = 1
        let z = Complex::new(0.0, 0.0);
        let result = z.exp();
        assert_complex_eq(&result, &Complex::new(1.0, 0.0));
    }

    #[test]
    fn test_edge_cases() {
        // Test zero
        let zero = Complex::new(0.0, 0.0);
        assert_eq!(zero.magnitude(), 0.0);

        // Test multiplication by zero
        let z = Complex::new(1.0, 1.0);
        let result = z.multiply(&zero);
        assert_complex_eq(&result, &zero);

        // Test adding zero
        let result = z.add(&zero);
        assert_complex_eq(&result, &z);
    }

    #[test]
    fn test_norm() {
        let z = Complex::new(3.0, 4.0);
        assert!((z.norm() - 5.0).abs() < EPSILON);
    }
}
