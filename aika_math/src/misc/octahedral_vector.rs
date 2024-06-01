use std::marker::PhantomData;
use cgmath::{BaseFloat, InnerSpace, Vector3};
use num_traits::clamp;
use crate::utils::{get_float_sign, get_vector3_one, get_vector3_zero};

/// F: Float type
/// D: Storage type, can be u16 for quantized float storage, or f32/f64 for regular float types
pub struct OctahedralVector<F, D> {
    pub x: D,
    pub y: D,
    _float_phantom: PhantomData<F>,
}

pub trait OctahedralStorageEncoderDecoder<F, D> {
    fn encode(value: F) -> D;

    fn decode(x: D, y: D) -> Vector3<F>;
}

pub trait OctahedralStorageType<F>: Sized {
    type EncodeDecoder: OctahedralStorageEncoderDecoder<F, Self>;
}

pub struct U16EncoderDecoder<F> {
    _float_phantom: PhantomData<F>,
}

impl<F> OctahedralStorageEncoderDecoder<F, u16> for U16EncoderDecoder<F> where F: BaseFloat {
    fn encode(value: F) -> u16 {
        assert!(value >= -F::one());
        assert!(value <= F::one());
        let two = F::from(2).unwrap();
        let value = clamp((value + F::one()) / two, F::zero(), F::one());
        let value = value * F::from(65535).unwrap();
        let value = value.round();
        let temp = value.to_u16().unwrap();
        temp
    }

    fn decode(x: u16, y: u16) -> Vector3<F> {
        let two = F::from(2).unwrap();
        let m = F::from(65535).unwrap();
        let mut v = get_vector3_zero();
        v.x = -F::one() + two * (F::from(x).unwrap() / m);
        v.y = -F::one() + two * (F::from(y).unwrap() / m);
        v.z = F::one() - (v.x.abs() + v.y.abs());

        if v.z < F::zero() {
            let xo = v.x;
            v.x = (F::one() - v.y.abs()) * get_float_sign(xo, false);
            v.y = (F::one() - xo.abs()) * get_float_sign(v.y, false);
        }

        v.normalize()
    }
}

impl<F> OctahedralStorageType<F> for u16 where F: BaseFloat {
    type EncodeDecoder = U16EncoderDecoder<F>;
}

/// F1: float type to be encoded
/// F2: float type encoded
pub struct FloatEncoderDecoder<F1, F2> {
    _float_phantom: PhantomData<F1>,
    _float_phantom1: PhantomData<F2>,
}

impl<F1, F2> OctahedralStorageEncoderDecoder<F1, F2> for FloatEncoderDecoder<F1, F2> where F1: BaseFloat, F2: BaseFloat {
    fn encode(value: F1) -> F2 {
        F2::from(value).unwrap()
    }

    fn decode(x: F2, y: F2) -> Vector3<F1> {
        let mut v = get_vector3_zero();
        v.x = F1::from(x).unwrap();
        v.y = F1::from(y).unwrap();
        v.z = F1::one() - (v.x.abs() + v.y.abs());

        if v.z < F1::zero() {
            let xo = v.x;
            v.x = (F1::one() - v.y.abs()) * get_float_sign(xo, false);
            v.y = (F1::one() - xo.abs()) * get_float_sign(v.y, false);
        }

        v.normalize()
    }
}

impl<F1> OctahedralStorageType<F1> for f32 where F1: BaseFloat {
    type EncodeDecoder = FloatEncoderDecoder<F1, f32>;
}

impl<F1> OctahedralStorageType<F1> for f64 where F1: BaseFloat {
    type EncodeDecoder = FloatEncoderDecoder<F1, f64>;
}

impl<F, D> OctahedralVector<F, D>
where
    D: OctahedralStorageType<F> + Copy,
    F: BaseFloat
{
    pub fn from_vector3(vec: Vector3<F>) -> Self {
        let l1_norm = vec.x.abs() + vec.y.abs() + vec.z.abs();
        assert!(l1_norm > F::zero());
        let point_on_octahedral = vec / l1_norm;

        if point_on_octahedral.z >= F::zero() {
            let x = D::EncodeDecoder::encode(point_on_octahedral.x);
            let y = D::EncodeDecoder::encode(point_on_octahedral.y);
            Self {
                x, y,
                _float_phantom: PhantomData,
            }
        } else {
            let x = D::EncodeDecoder::encode(
                (F::one() - point_on_octahedral.y.abs()) * get_float_sign(point_on_octahedral.x, false)
            );
            let y = D::EncodeDecoder::encode(
                (F::one() - point_on_octahedral.x.abs()) * get_float_sign(point_on_octahedral.y, false)
            );
            Self {
                x, y,
                _float_phantom: PhantomData,
            }
        }
    }

    pub fn to_vector3(&self) -> Vector3<F> {
        D::EncodeDecoder::decode(self.x, self.y)
    }
}
