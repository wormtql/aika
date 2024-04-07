#[macro_export]
macro_rules! f {
    ($v:expr) => {
        F::from($v).unwrap()
    }
}
