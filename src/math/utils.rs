pub fn approx_eq(x: &f64, y: &f64) -> bool {
    (x-y).abs() <= 1e-10
}