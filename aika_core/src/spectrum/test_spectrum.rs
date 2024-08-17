use crate::spectrum::{CIE_Y_INTEGRAL_F32, CIE_Y_INTEGRAL_F64, ConstantSpectrum, SampledSpectrum, XYZ};
use crate::spectrum::spectrum_utils::SpectrumUtils;

#[test]
fn test_xyz1() {
    let const_spectrum = ConstantSpectrum::new(1.0);
    let xyz = SpectrumUtils::spectrum_to_XYZ(&const_spectrum);
    let diff = (xyz.y - CIE_Y_INTEGRAL_F64).abs();
    assert!(diff < 1e-4);

    let const_spectrum = ConstantSpectrum::new(1.0_f32);
    let xyz = SpectrumUtils::spectrum_to_XYZ(&const_spectrum);
    let diff = (xyz.y - CIE_Y_INTEGRAL_F32).abs();
    assert!(diff < 1e-4);
}
