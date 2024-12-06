use std::ops::Range;

use tailcall::tailcall;

pub fn f(x: f64) -> f64 {
    (-6_f64).mul_add(x.powi(2), x.powi(3)) + 11_f64.mul_add(x, -6.)
}

pub fn solve_for(interval @ Range { start: a, end: b }: Range<f64>, eps: f64) -> Result<f64, f64> {
    let result = actual_solve_for(interval, eps);

    if a <= b && f(a) * f(b) < 0. {
        Ok(result)
    } else {
        Err(result)
    }
}

#[allow(unreachable_code)]
#[tailcall]
fn actual_solve_for(interval: Range<f64>, eps: f64) -> f64 {
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

    actual_solve_for(interval, eps)
}
