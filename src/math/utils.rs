pub fn approx_eq(x: &f64, y: &f64) -> bool {
    (x-y).abs() <= 1e-10
}

#[inline]
/// Returns the maximum of the given iterator of floats. If the iterator is empty, returns 0.0.
pub fn max(iter: impl Iterator<Item=f64>) -> f64 {
    iter.fold(0.0, f64::max)
}

#[inline]
/// Returns the maximum absolute value of the given iterator of floats. If the iterator is empty, returns 0.0.
pub fn max_abs<'a>(iter: impl Iterator<Item=&'a f64>) -> f64 {
    iter.fold(0.0, |acc, x| f64::max(acc, x.abs()))
}