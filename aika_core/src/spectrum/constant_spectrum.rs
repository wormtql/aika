use cgmath::BaseFloat;
use crate::spectrum::Spectrum;

pub struct ConstantSpectrum<F> {
    pub value: F,
}

impl<F> ConstantSpectrum<F> where F: BaseFloat {
    pub fn new(value: F) -> Self {
        Self {
            value
        }
    }
}

impl<F> Spectrum<F> for ConstantSpectrum<F> where F: BaseFloat {
    fn get_value(&self, lambda: F) -> F {
        self.value
    }

    fn max_value(&self) -> F {
        self.value
    }
}
