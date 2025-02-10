use linear_algebra::f64_operations::Operations;
use std::f64::consts::PI;

pub fn quadratic(inputs: [f64; 3]) -> Vec<f64> {
    let p = inputs[1] / (2.0 * inputs[2]);
    let q = inputs[0] / inputs[2];

    let d = p * p - q;

    if d.zero() {
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

    let squared_a = a * a;
    let p = 1.0 / 3.0 * (-1.0 / 3.0 * squared_a + b);
    let q = 1.0 / 2.0 * (2.0 / 27.0 * a * squared_a - 1.0 / 3.0 * a * b + c);

    let cubic_p = p * p * p;
    let d = q * q + cubic_p;

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

    let squared_a =  a * a;
    let p = (-3.0 / 8.0) * squared_a + b;
    let q = (1.0 / 8.0) * squared_a * a - (1.0 / 2.0) * a * b + c;
    let r = (-3.0 / 256.0) * squared_a * squared_a + (1.0 / 16.0) * squared_a * b - (1.0 / 4.0) * a * c + d;

    let mut solutions = vec![];

    if r.zero() { /* no absolute term: y(y^3 + py + q) = 0 */
        let coefficients = [q, p, 0.0, 1.0];
        solutions.append(&mut cubic(coefficients));
        solutions.push(0.0);
    } else { /* solve the resolvent cubic ... */
        let coefficients = [
            (1.0 / 2.0) * r * p - (1.0 / 8.0) * q * q,
            -r,
            (-1.0 / 2.0) * p,
            1.0
        ];

        solutions.append(&mut cubic(coefficients));

        let z = solutions[0];

        let u = match handle_result(z * z - r) {
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

    let substitute = (1.0 / 4.0) * a;

    solutions.iter()
        .map(|s| s - substitute)
        .collect()
}

fn determine_solutions(d: f64, p: f64, q: f64, cubic_p: f64) -> Vec<f64> {
    if d.zero() {
        if q.zero() { /* one triple solution */
            return vec![0.0];
        }
        /* one single and one double solution */
        let u = -q.cbrt();
        return vec![2.0 * u, -u];
    } else if d < 0.0 { /* Casus irreducibilis: three real solutions */
        let phi = 1.0 / 3.0 * (-q / (-cubic_p).sqrt()).acos();
        let t = 2.0 * (-p).sqrt();

        return vec![
            t * phi.cos(),
            -t * (phi + PI / 3.0).cos(),
            -t * (phi - PI / 3.0).cos()
        ];
    }
    /* one real solution */
    let sqrt_d = d.sqrt();
    let u = (sqrt_d - q).cbrt();
    let v = -(sqrt_d + q).cbrt();

    vec![u + v]
}

fn handle_result(x: f64) -> Option<f64> {
    if x.zero() {
        return Some(0.0);
    } else if x > 0.0 {
        return Some(x.sqrt());
    }
    Some(0.0)
}