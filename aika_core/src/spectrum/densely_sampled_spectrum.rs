use cgmath::BaseFloat;
use crate::spectrum::{MAX_LAMBDA, MIN_LAMBDA, Spectrum};

pub struct DenselySampledSpectrum<F> {
    pub lambda_min: F,
    pub lambda_max: F,
    pub values: Vec<F>,
    max_value: F,
}

impl<F> DenselySampledSpectrum<F> where F: BaseFloat {
    pub fn from_spectrum<S: Spectrum<F>>(s: &S, min: usize, max: usize) -> Self {
        let mut values = Vec::new();
        let mut max_value = F::neg_infinity();
        for i in min..=max {
            values.push(s.get_value(F::from(i).unwrap()));
            max_value = max_value.max(values[i - min]);
        }

        Self {
            lambda_min: F::from(min).unwrap(),
            lambda_max: F::from(max).unwrap(),
            values,
            max_value,
        }
    }

    pub fn from_arr<F2: BaseFloat>(arr: &[F2]) -> Self {
        let mut values = Vec::new();
        let mut max_value = F::neg_infinity();
        for i in MIN_LAMBDA..=MAX_LAMBDA {
            values.push(F::from(arr[i - MIN_LAMBDA]).unwrap());
            max_value = max_value.max(values[i - MIN_LAMBDA]);
        }
        Self {
            lambda_min: F::from(MIN_LAMBDA).unwrap(),
            lambda_max: F::from(MAX_LAMBDA).unwrap(),
            values,
            max_value
        }
    }
}

impl<F> Spectrum<F> for DenselySampledSpectrum<F> where F: BaseFloat {
    fn get_value(&self, lambda: F) -> F {
        if lambda < self.lambda_min || lambda > self.lambda_max {
            F::zero()
        } else {
            let offset = (lambda - self.lambda_min).to_usize().unwrap();
            self.values[offset]
        }
    }

    fn max_value(&self) -> F {
        self.max_value
    }
}
