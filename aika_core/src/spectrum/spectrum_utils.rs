use std::marker::PhantomData;
use cgmath::BaseFloat;
use crate::spectrum::{CIE_X_RAW_F32, CIE_Y_RAW_F32, CIE_Z_RAW_F32, MAX_LAMBDA, MIN_LAMBDA, SAMPLE_COUNT, SampledSpectrum, SampledWavelength, Spectrum, XYZ};

pub struct SpectrumUtils<F> {
    _phantom: PhantomData<F>,
}

impl<F: BaseFloat> SpectrumUtils<F> {
    pub fn sample_discrete_wavelength<S: Spectrum<F>>(s: &S, wavelengths: &SampledWavelength<F, F>) -> SampledSpectrum<F> {
        let mut values = [F::zero(); SAMPLE_COUNT];
        for i in 0..SAMPLE_COUNT {
            values[i] = s.get_value(wavelengths[i]);
        }
        SampledSpectrum::new(values)
    }

    pub fn inner_product_spectrum<S1: Spectrum<F>, S2: Spectrum<F>>(s1: &S1, s2: &S2) -> F {
        let mut integral = F::zero();
        for i in MIN_LAMBDA..=MAX_LAMBDA {
            let lambda = F::from(i).unwrap();
            integral += s1.get_value(lambda) * s2.get_value(lambda);
        }
        integral
    }

    pub fn inner_product_spectrum_and_arr<S: Spectrum<F>, F2: BaseFloat>(s1: &S, arr: &[F2]) -> F {
        let mut integral = F::zero();
        for i in MIN_LAMBDA..=MAX_LAMBDA {
            let lambda = F::from(i).unwrap();
            integral += s1.get_value(lambda) * F::from(arr[i - MIN_LAMBDA]).unwrap();
        }
        integral
    }

    pub fn spectrum_to_xyz<S: Spectrum<F>>(s: &S) -> XYZ<F> {
        let x = Self::inner_product_spectrum_and_arr(s, &CIE_X_RAW_F32);
        let y = Self::inner_product_spectrum_and_arr(s, &CIE_Y_RAW_F32);
        let z = Self::inner_product_spectrum_and_arr(s, &CIE_Z_RAW_F32);
        XYZ::new(x, y, z)
    }
}
