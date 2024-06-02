use std::ops::Index;
use cgmath::BaseFloat;
use crate::spectrum::{SAMPLE_COUNT, SampledSpectrum};

/// W: Wavelength type
/// F: pdf type
pub struct SampledWavelength<F, W> {
    pub wavelengths: [W; SAMPLE_COUNT],
    pub pdf: [F; SAMPLE_COUNT],
}

impl<F, W> SampledWavelength<F, W> {
    pub fn new(wavelengths: [W; SAMPLE_COUNT], pdf: [F; SAMPLE_COUNT]) -> Self {
        Self {
            wavelengths,
            pdf,
        }
    }
}

impl<F, W> SampledWavelength<F, W>
where F: BaseFloat
{
    /// Return pdfs as SampledSpectrum, note the SampledSpectrum is only used as a container
    /// it has nothing to do with "Spectrum"
    pub fn get_pdf(&self) -> SampledSpectrum<F> {
        SampledSpectrum::new(self.pdf.clone())
    }
}

impl<F, W> SampledWavelength<F, W> where F: BaseFloat, W: BaseFloat {
    pub fn to_usize(&self) -> SampledWavelength<F, usize> {
        let mut w = [0; SAMPLE_COUNT];
        for i in 0..SAMPLE_COUNT {
            w[i] = self.wavelengths[i].round().to_usize().unwrap()
        }
        SampledWavelength::new(w, self.pdf.clone())
    }
}

impl<F, W: Copy> Index<usize> for SampledWavelength<F, W> {
    type Output = W;

    fn index(&self, index: usize) -> &Self::Output {
        &self.wavelengths[index]
    }
}
