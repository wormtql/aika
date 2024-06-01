use crate::utils::{F32_MAX_VALUE_BELOW_ONE, F32_MIN_VALUE_ABOVE_ONE, F32_MIN_VALUE_ABOVE_ZERO, F64_MAX_VALUE_BELOW_ONE, F64_MIN_VALUE_ABOVE_ONE, F64_MIN_VALUE_ABOVE_ZERO, next_float_down, next_float_up};

#[test]
fn test_next_float_down() {
    let f = 1.0_f64;
    let d = next_float_down(f);
    assert_eq!(d, F64_MAX_VALUE_BELOW_ONE);

    let f = 1.0_f32;
    let d = next_float_down(f);
    assert_eq!(d, F32_MAX_VALUE_BELOW_ONE);

    let f = F64_MIN_VALUE_ABOVE_ONE;
    assert_eq!(1.0_f64, next_float_down(f));

    let inf = f64::INFINITY;
    assert_eq!(f64::MAX, next_float_down(inf));

    let inf = f64::NEG_INFINITY;
    assert_eq!(f64::NEG_INFINITY, next_float_down(inf));
}

#[test]
fn test_next_float_up() {
    let f = 1.0_f64;
    let d = next_float_up(f);
    assert_eq!(d, F64_MIN_VALUE_ABOVE_ONE);

    let f = 1.0_f32;
    let d = next_float_up(f);
    assert_eq!(d, F32_MIN_VALUE_ABOVE_ONE);
}

#[test]
fn test_min_value_above_zero() {
    let f = F64_MIN_VALUE_ABOVE_ZERO;
    assert!(f > 0.0_f64);
    assert_eq!(0.0, next_float_down(f));

    let f = F32_MIN_VALUE_ABOVE_ZERO;
    assert!(f > 0.0);
    assert_eq!(0.0, next_float_down(f));
}