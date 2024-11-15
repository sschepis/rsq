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
}
