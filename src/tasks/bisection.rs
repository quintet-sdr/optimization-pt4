use std::ops::Range;

use tailcall::tailcall;

pub fn f(x: f64) -> f64 {
    (-6_f64).mul_add(x.powi(2), x.powi(3)) + 11_f64.mul_add(x, -6.)
}

pub fn solve_for(interval @ Range { start: a, end: b }: Range<f64>, eps: f64) -> Result<f64, f64> {
    let root = actual_solve_for(interval, eps);

    if a <= b && f(a) * f(b) < 0. {
        // Signify that the root is valid.
        Ok(root)
    } else {
        // Signify that the root is probably invalid.
        Err(root)
    }
}

#[allow(unreachable_code)]
// Tail recursion optimization.
#[tailcall]
fn actual_solve_for(interval: Range<f64>, eps: f64) -> f64 {
    // Extract the ends of the interval to more convenient names.
    let Range { start: a, end: b } = interval;

    let c = (a + b) / 2.;

    if f(c).abs() < eps {
        return c;
    }

    let interval = if f(c).signum() == f(a).signum() {
        c..b
    } else {
        a..c
    };

    // Go to the next iteration.
    actual_solve_for(interval, eps)
}
