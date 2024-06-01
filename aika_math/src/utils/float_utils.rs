use std::any::TypeId;
use cgmath::BaseFloat;
use crate::utils::type_cast_same_size;

// pub trait FloatExtensions {
    // fn
// }

pub fn next_f32_down(value: f32) -> f32 {
    if value.is_infinite() && value < 0.0 {
        value
    } else if value == 0.0 {
        -0.0
    } else {
        let mut ui = type_cast_same_size::<f32, u32>(value);
        if value >= 0.0 {
            ui -= 1;
        } else {
            ui += 1;
        }
        type_cast_same_size::<u32, f32>(ui)
    }
}

pub fn next_f32_up(value: f32) -> f32 {
    if value.is_infinite() && value > 0.0 {
        value
    } else if value == -0.0 {
        0.0
    } else {
        let mut ui = float_to_u32(value);
        if value >= 0.0 {
            ui += 1;
        } else {
            ui -= 1;
        }
        type_cast_same_size::<u32, f32>(ui)
    }
}

pub fn next_f64_down(value: f64) -> f64 {
    if value.is_infinite() && value < 0.0 {
        value
    } else if value == 0.0 {
        -0.0
    } else {
        let mut ui = type_cast_same_size::<f64, u64>(value);
        if value >= 0.0 {
            ui -= 1;
        } else {
            ui += 1;
        }
        type_cast_same_size::<u64, f64>(ui)
    }
}

pub fn next_f64_up(value: f64) -> f64 {
    if value.is_infinite() && value > 0.0 {
        value
    } else if value == -0.0 {
        0.0
    } else {
        let mut ui = type_cast_same_size::<f64, u64>(value);
        if value >= 0.0 {
            ui += 1;
        } else {
            ui -= 1;
        }
        type_cast_same_size::<u64, f64>(ui)
    }
}

/// Get the next higher float value
/// If the value is +infinity, return itself
pub fn next_float_up<F: BaseFloat + 'static>(value: F) -> F {
    if TypeId::of::<F>() == TypeId::of::<f32>() {
        let f = next_f32_up(value.to_f32().unwrap());
        F::from(f).unwrap()
    } else if TypeId::of::<F>() == TypeId::of::<f64>() {
        let f = next_f64_up(value.to_f64().unwrap());
        F::from(f).unwrap()
    } else {
        unreachable!()
    }
}

pub fn next_float_down<F: BaseFloat + 'static>(value: F) -> F {
    if TypeId::of::<F>() == TypeId::of::<f32>() {
        let f = next_f32_down(value.to_f32().unwrap());
        F::from(f).unwrap()
    } else if TypeId::of::<F>() == TypeId::of::<f64>() {
        let f = next_f64_down(value.to_f64().unwrap());
        F::from(f).unwrap()
    } else {
        unreachable!()
    }
}

pub fn float_to_u32(value: f32) -> u32 {
    unsafe {
        let ptr = &value as *const f32 as *const u32;
        *ptr
    }
}

pub fn is_f32<F: 'static>() -> bool {
    TypeId::of::<F>() == TypeId::of::<f32>()
}

pub fn is_f64<F: 'static>() -> bool {
    TypeId::of::<F>() == TypeId::of::<f64>()
}

pub const fn construct_f64_from_components(sign: u64, exponent: u64, fraction: u64) -> f64 {
    let mut d: u64 = 0;
    d = d | (sign << 63);
    d = d | (exponent << 52);
    d = d | fraction;
    unsafe {
        let ptr = &d as *const u64 as *const f64;
        *ptr
    }
}

pub const fn construct_f32_from_components(sign: u32, exponent: u32, fraction: u32) -> f32 {
    let mut d: u32 = 0;
    d = d | (sign << 31) | (exponent << 23) | fraction;
    unsafe {
        let ptr = &d as *const u32 as *const f32;
        *ptr
    }
}

pub const F64_MAX_VALUE_BELOW_ONE: f64 = f64::from_bits(0b0_01111111110_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111);
pub const F32_MAX_VALUE_BELOW_ONE: f32 = f32::from_bits(0b0_01111110_1111_1111_1111_1111_1111_111);

pub fn get_max_value_below_one<F: BaseFloat + 'static>() -> F {
    if TypeId::of::<F>() == TypeId::of::<f32>() {
        F::from(F32_MAX_VALUE_BELOW_ONE).unwrap()
    } else if TypeId::of::<F>() == TypeId::of::<f64>() {
        F::from(F64_MAX_VALUE_BELOW_ONE).unwrap()
    } else {
        unreachable!()
    }
}

pub const F64_MIN_VALUE_ABOVE_ONE: f64 = construct_f64_from_components(0, 1023, 1);
pub const F32_MIN_VALUE_ABOVE_ONE: f32 = construct_f32_from_components(0, 127, 1);

pub fn get_min_value_above_one<F: BaseFloat + 'static>() -> F {
    if TypeId::of::<F>() == TypeId::of::<f32>() {
        F::from(F32_MIN_VALUE_ABOVE_ONE).unwrap()
    } else if TypeId::of::<F>() == TypeId::of::<f64>() {
        F::from(F64_MIN_VALUE_ABOVE_ONE).unwrap()
    } else {
        unreachable!()
    }
}

pub const F64_MIN_VALUE_ABOVE_ZERO: f64 = construct_f64_from_components(0, 0, 1);
pub const F32_MIN_VALUE_ABOVE_ZERO: f32 = construct_f32_from_components(0, 0, 1);

pub fn get_min_value_above_zero<F: BaseFloat + 'static>() -> F {
    if is_f32::<F>() {
        F::from(F32_MIN_VALUE_ABOVE_ZERO).unwrap()
    } else if is_f64::<F>() {
        F::from(F64_MIN_VALUE_ABOVE_ZERO).unwrap()
    } else {
        unreachable!()
    }
}

pub fn get_float_sign<F: BaseFloat>(value: F, include_zero: bool) -> F {
    assert!(!value.is_nan());
    if include_zero && (value == F::zero() || value == F::neg_zero()) {
        return F::zero();
    }
    if value > F::zero() || value == F::zero() {
        F::one()
    } else if value < F::zero() || value == F::neg_zero() {
        -F::one()
    } else {
        unreachable!()
    }
}
