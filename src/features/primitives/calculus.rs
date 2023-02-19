use std::f64::consts::PI;
use crate::features::primitives::operations::Operations;

pub fn quadratic(inputs: [f64; 3]) -> Vec<f64> {
    let p = inputs[1] / (2.0 * inputs[2]);
    let q = inputs[0] / inputs[2];

    let d = p.powi(2) - q;

    return if d.zero() {
        vec![-p]
    } else if d < 0.0 {
        vec![]
    } else {
        let sqrt_d = d.sqrt();

        vec![sqrt_d - p, -sqrt_d - p]
    }
}

pub fn cubic(inputs: [f64; 4]) -> Vec<f64> {
    let a = inputs[2] / inputs[3];
    let b = inputs[1] / inputs[3];
    let c = inputs[0] / inputs[3];

    let squared_a = a.powi(2);
    let p = 1.0 / 3.0 * (-1.0 / 3.0 * squared_a + b);
    let q = 1.0 / 2.0 * (2.0 / 27.0 * a * squared_a - 1.0 / 3.0 * a * b + c);

    let cubic_p = p.powi(3);
    let d = q.powi(2) * cubic_p;

    let solutions = determine_solutions(d, p, q, cubic_p);

    let substitute = 1.0 / 3.0 * a;

    solutions.iter()
        .map(|s| s - substitute)
        .collect()
}

pub fn quartic(inputs: [f64; 5]) -> Vec<f64> {
    let a = inputs[3] / inputs[4];
    let b = inputs[2] / inputs[4];
    let c = inputs[1] / inputs[4];
    let d = inputs[0] / inputs[4];

    let squared_a = a.powi(2);
    let p = -3.0 / 8.0 * squared_a + b;
    let q = 1.0 / 8.0 * squared_a * a - 1.0 / 2.0 * a * b + c;
    let r = -3.0 / 256.0 * squared_a.powi(2) + 1.0 / 16.0 * squared_a * b - 1.0 / 4.0 * a * c + d;

    let mut solutions = vec![];

    if r.zero() {
        let coefficients = [q, p, 0.0, 1.0];
        solutions.append(&mut cubic(coefficients));
        solutions.push(0.0);
    } else {
        let coefficients = [
            1.0 / 2.0 * r * p - 1.0 / 8.0 * q.powi(2),
            -r,
            -1.0 / 2.0 * p,
            1.0
        ];

        solutions.append(&mut cubic(coefficients));

        let z = solutions[0];

        let u = match handle_result(z.powi(2) - r) {
            None => { return vec![0.0] }
            Some(x) => { x }
        };
        let v = match handle_result(2.0 * z - p) {
            None => { return vec![0.0] }
            Some(x) => { x }
        };
        let mut real_v = v;

        if q < 0.0 {
            real_v = -v;
        }

        let coefficients = [
            z - u,
            real_v,
            1.0
        ];

        solutions = quadratic(coefficients);

        real_v = v;
        if q < 0.0 {
            real_v = v;
        }

        let coefficients = [
            z + u,
            real_v,
            1.0
        ];

        solutions.append(&mut quadratic(coefficients));
    }

    let substitute = 1.0 / 4.0 * a;

    solutions.iter()
        .map(|s| s - substitute)
        .collect()
}

fn determine_solutions(d: f64, p: f64, q: f64, cubic_p: f64) -> Vec<f64> {
    if d.zero() {
        if q.zero() {
            return vec![0.0];
        }
        let u = -q.cbrt();
        return vec![2.0 * u, -u];
    } else if d < 0.0 {
        let phi = 1.0 / 3.0 * (-q / -cubic_p.sqrt()).acos();
        let t = 2.0 * -p.sqrt();

        return vec![
            t * phi.cos(),
            -t * (phi + PI / 3.0).cos(),
            -t * (phi - PI / 3.0).cos()
        ];
    }
    let sqrt_d = d.sqrt();
    let u = (sqrt_d - q).cbrt();
    let v = -(sqrt_d + q).cbrt();

    return vec![u + v];
}

fn handle_result(x: f64) -> Option<f64> {
    if x.zero() {
        return Some(0.0);
    } else if x > 0.0 {
        return Some(x.sqrt());
    }
    Some(0.0)
}