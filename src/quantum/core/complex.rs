use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Complex {
        Complex { real, imag }
    }

    pub fn from_polar(r: f64, theta: f64) -> Complex {
        Complex {
            real: r * theta.cos(),
            imag: r * theta.sin(),
        }
    }

    pub fn norm(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    pub fn norm_sqr(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    pub fn magnitude(&self) -> f64 {
        self.norm()
    }

    pub fn arg(&self) -> f64 {
        self.imag.atan2(self.real)
    }

    pub fn conjugate(&self) -> Complex {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }

    pub fn exp(&self) -> Complex {
        let r = self.real.exp();
        Complex {
            real: r * self.imag.cos(),
            imag: r * self.imag.sin(),
        }
    }

    pub fn mul(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }

    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl Mul for &Complex {
    type Output = Complex;

    fn mul(self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_complex_add() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a + b;
        assert_eq!(c.real, 4.0);
        assert_eq!(c.imag, 6.0);
    }

    #[test]
    fn test_complex_mul() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a * b;
        assert_eq!(c.real, -5.0);
        assert_eq!(c.imag, 10.0);
    }

    #[test]
    fn test_from_polar() {
        let c = Complex::from_polar(2.0, PI/4.0);
        assert!((c.real - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!((c.imag - 2.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_norm() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.norm(), 5.0);
        assert_eq!(c.norm_sqr(), 25.0);
    }

    #[test]
    fn test_conjugate() {
        let c = Complex::new(1.0, 2.0);
        let conj = c.conjugate();
        assert_eq!(conj.real, 1.0);
        assert_eq!(conj.imag, -2.0);
    }

    #[test]
    fn test_exp() {
        let c = Complex::new(0.0, PI);
        let exp = c.exp();
        assert!((exp.real + 1.0).abs() < 1e-10);
        assert!(exp.imag.abs() < 1e-10);
    }
}
