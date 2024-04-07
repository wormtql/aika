use std::ops::{Add, Div, Mul, Sub};
use cgmath::BaseFloat;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Complex<F> {
    pub real: F,
    pub imaginary: F,
}

impl<F> Complex<F> where F: BaseFloat {
    pub fn new(real: F, imaginary: F) -> Self {
        Complex {
            real, imaginary
        }
    }

    pub fn new_real(real: F) -> Self {
        Complex {
            real,
            imaginary: F::zero()
        }
    }

    pub fn square_length(&self) -> F {
        self.real.powi(2) + self.imaginary.powi(2)
    }

    pub fn length(&self) -> F {
        self.square_length().sqrt()
    }

    pub fn inverse(&self) -> Complex<F> {
        let length2 = self.square_length();
        Complex {
            real: self.real / length2,
            imaginary: -self.imaginary / length2
        }
    }

    pub fn sqrt(&self) -> Complex<F> {
        let n = self.length();
        let t1 = (F::from(0.5).unwrap() * (n + self.real.abs())).sqrt();
        let t2 = F::from(0.5).unwrap() * self.imaginary / t1;

        if n == F::zero() {
            return Complex {
                real: F::zero(),
                imaginary: F::zero()
            }
        }

        if self.real >= F::zero() {
            Complex::new(t1, t2)
        } else {
            if self.imaginary >= F::zero() {
                Complex::new(t2.abs(), t1.abs())
            } else {
                Complex::new(t2.abs(), -t1.abs())
            }
        }
    }
}

impl<F> Mul for Complex<F> where F: BaseFloat {
    type Output = Complex<F>;

    fn mul(self, rhs: Self) -> Self::Output {
        let real = self.real * rhs.real - self.imaginary * rhs.imaginary;
        let imaginary = self.real * rhs.imaginary + self.imaginary * rhs.real;
        Complex {
            real, imaginary
        }
    }
}

impl<F> Mul<F> for Complex<F> where F: BaseFloat {
    type Output = Complex<F>;

    fn mul(self, rhs: F) -> Self::Output {
        Complex {
            real: self.real * rhs,
            imaginary: self.imaginary * rhs
        }
    }
}

impl<F> Div for Complex<F> where F: BaseFloat {
    type Output = Complex<F>;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl<F> Div<F> for Complex<F> where F: BaseFloat {
    type Output = Complex<F>;

    fn div(self, rhs: F) -> Self::Output {
        Complex {
            real: self.real / rhs,
            imaginary: self.imaginary / rhs
        }
    }
}

impl<F> Sub for Complex<F> where F: BaseFloat {
    type Output = Complex<F>;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary
        }
    }
}

impl<F> Add for Complex<F> where F: BaseFloat {
    type Output = Complex<F>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary
        }
    }
}
