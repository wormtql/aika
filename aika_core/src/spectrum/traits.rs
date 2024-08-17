use cgmath::BaseFloat;
use crate::spectrum::SampledWavelength;

/// Minimum visible light wavelength
pub const MIN_LAMBDA: usize = 360;
/// Maximum visible light wavelength
pub const MAX_LAMBDA: usize = 830;

/// The trait for the spectrum
pub trait Spectrum<F: BaseFloat> {
    /// Get the SPD value given a wavelength in nm
    /// The returned value may be in any physical unit, for example radiance or irradiance
    fn get_value(&self, lambda: F) -> F;

    /// Get the upper bound of the SPD
    fn max_value(&self) -> F;
    // fn sample(&self, wavelengths: &SampledWavelength<F>)
}
