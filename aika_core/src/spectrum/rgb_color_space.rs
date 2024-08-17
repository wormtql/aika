use cgmath::{BaseFloat, Matrix3, SquareMatrix, Vector2};
use crate::spectrum::{DenselySampledSpectrum, RGB, XYZ};
use crate::spectrum::spectrum_utils::SpectrumUtils;
use anyhow::{anyhow, Result};

pub struct RGBColorSpace<F> {
    pub r: Vector2<F>,
    pub g: Vector2<F>,
    pub b: Vector2<F>,
    /// Chromaticity of white point
    pub w: Vector2<F>,
    /// Dense spectrum of white point
    pub illuminant: DenselySampledSpectrum<F>,

    XYZ_from_RGB: Matrix3<F>,
    RGB_from_XYZ: Matrix3<F>,
}

impl<F> RGBColorSpace<F>
where
    F: BaseFloat
{
    pub fn new(r: Vector2<F>, g: Vector2<F>, b: Vector2<F>, illuminant: DenselySampledSpectrum<F>) -> Result<Self> {
        let white_point_XYZ = SpectrumUtils::spectrum_to_XYZ(&illuminant);
        let white_point_chromaticity = white_point_XYZ.xy();

        let r_XYZ = XYZ::from_xyY(r, F::one());
        let g_XYZ = XYZ::from_xyY(g, F::one());
        let b_XYZ = XYZ::from_xyY(b, F::one());

        let rgb_matrix = Matrix3::from_cols(r_XYZ.to_vector3(), b_XYZ.to_vector3(), g_XYZ.to_vector3());
        let rgb_matrix_inv = match rgb_matrix.invert() {
            Some(v) => v,
            None => return Err(anyhow!("The RGB Matrix cannot be inverted, RGB should be linearly independent"))
        };
        let C = rgb_matrix_inv * white_point_XYZ.to_vector3();
        let XYZ_from_RGB = rgb_matrix * Matrix3::from_diagonal(C);
        let RGB_from_XYZ = XYZ_from_RGB.invert().unwrap();

        Ok(Self {
            r, g, b,
            w: white_point_chromaticity,
            illuminant,
            XYZ_from_RGB,
            RGB_from_XYZ,
        })
    }

    pub fn XYZ_to_RGB(&self, xyz: XYZ<F>) -> RGB<F> {
        let result = self.RGB_from_XYZ * xyz.to_vector3();
        RGB::new(result.x, result.y, result.z)
    }

    pub fn RGB_to_XYZ(&self, rgb: RGB<F>) -> XYZ<F> {
        let result = self.XYZ_from_RGB * rgb.to_vector3();
        XYZ::new(result.x, result.y, result.z)
    }
}
