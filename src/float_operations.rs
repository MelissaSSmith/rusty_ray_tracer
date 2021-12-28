pub fn equals(a: f64, b: f64) -> bool {
    let epsilon = 0.00001;
    let diff = a - b;
    diff.abs() < epsilon
}