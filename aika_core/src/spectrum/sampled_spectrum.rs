use std::any::TypeId;
use std::ops::{Add, Div, Index, Mul, Sub};
use cgmath::BaseFloat;
use crate::spectrum::{CIE_X_RAW_F32, CIE_X_RAW_F64, CIE_Y_INTEGRAL_F32, SampledWavelength, XYZ};
use crate::spectrum::spectrum_utils::SpectrumUtils;

pub const SAMPLE_COUNT: usize = 4;

/// The sampled spectrum, only contains wavelength, pdf is saved in other place
#[derive(Copy, Eq, PartialEq, Clone, Hash)]
pub struct SampledSpectrum<F> {
    pub values: [F; SAMPLE_COUNT],
}

impl<F> SampledSpectrum<F> {
    pub fn new(values: [F; SAMPLE_COUNT]) -> Self {
        Self {
            values
        }
    }
}

impl<F: BaseFloat> SampledSpectrum<F> {
    pub fn average(&self) -> F {
        let mut sum = F::zero();
        for i in self.values {
            sum += i;
        }
        sum /= F::from(SAMPLE_COUNT).unwrap();
        sum
    }

    /// If divides by zero, the result is zero
    pub fn safe_div(&self, other: SampledSpectrum<F>) -> SampledSpectrum<F> {
        let mut values = [F::zero(); SAMPLE_COUNT];
        for i in 0..SAMPLE_COUNT {
            if other.values[i] == F::zero() {
                values[i] = F::zero();
            } else {
                values[i] = self.values[i] / other.values[i];
            }
        }
        SampledSpectrum::new(values)
    }

    /// Convert the sampled spectrum to XYZ color space using monte carlo integration
    pub fn to_xyz(&self, sampled_wavelengths: &SampledWavelength<F, F>) -> XYZ<F> {
        let mut result = [F::zero(); 3];
        let wavelengths_usize = sampled_wavelengths.to_usize();

        for i in 0..3 {
            let mut sum = F::zero();
            for i in 0..SAMPLE_COUNT {
                sum += F::from(CIE_X_RAW_F32[wavelengths_usize[i]]).unwrap() * self[i];
            }
            let pdf = sampled_wavelengths.pdf[i];
            let value = if pdf.is_zero() {
                F::zero()
            } else {
                sum / F::from(4).unwrap() / pdf / F::from(CIE_Y_INTEGRAL_F32).unwrap()
            };
            result[i] = value;
        }

        XYZ::new(result[0], result[1], result[2])
    }
}

impl<F: BaseFloat> Add for SampledSpectrum<F> {
    type Output = SampledSpectrum<F>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = [F::zero(); SAMPLE_COUNT];
        for i in 0..SAMPLE_COUNT {
            result[i] = self.values[i] + rhs.values[i];
        }
        Self {
            values: result
        }
    }
}

impl<F: BaseFloat> Sub for SampledSpectrum<F> {
    type Output = SampledSpectrum<F>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = [F::zero(); SAMPLE_COUNT];
        for i in 0..SAMPLE_COUNT {
            result[i] = self.values[i] - rhs.values[i];
        }
        Self {
            values: result
        }
    }
}

impl<F: BaseFloat> Mul for SampledSpectrum<F> {
    type Output = SampledSpectrum<F>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = [F::zero(); SAMPLE_COUNT];
        for i in 0..SAMPLE_COUNT {
            result[i] = self.values[i] * rhs.values[i];
        }
        Self {
            values: result
        }
    }
}

impl<F: BaseFloat> Mul<F> for SampledSpectrum<F> {
    type Output = SampledSpectrum<F>;

    fn mul(self, rhs: F) -> Self::Output {
        let mut result = [F::zero(); SAMPLE_COUNT];
        for i in 0..SAMPLE_COUNT {
            result[i] = self.values[i] * rhs;
        }
        Self {
            values: result
        }
    }
}

impl<F: BaseFloat> Div for SampledSpectrum<F> {
    type Output = SampledSpectrum<F>;

    fn div(self, rhs: Self) -> Self::Output {
        let mut result = [F::zero(); SAMPLE_COUNT];
        for i in 0..SAMPLE_COUNT {
            result[i] = self.values[i] / rhs.values[i];
        }
        Self {
            values: result
        }
    }
}

impl<F: BaseFloat> Div<F> for SampledSpectrum<F> {
    type Output = SampledSpectrum<F>;

    fn div(self, rhs: F) -> Self::Output {
        assert!(!rhs.is_zero());
        let mut result = [F::zero(); SAMPLE_COUNT];
        for i in 0..SAMPLE_COUNT {
            result[i] = self.values[i] / rhs;
        }
        Self {
            values: result
        }
    }
}

impl<F> Index<usize> for SampledSpectrum<F>
where
    F: Copy
{
    type Output = F;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}
