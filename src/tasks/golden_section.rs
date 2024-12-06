use std::cmp::Ordering;
use std::ops::Range;

use tailcall::tailcall;

fn f(x: f64) -> f64 {
    (x - 2.).mul_add(x - 2., 3.)
}

#[allow(unreachable_code)]
#[tailcall]
pub fn solve_for(interval: Range<f64>, eps: f64) -> (f64, f64) {
    const FRAC_1_PHI: f64 = 0.618_033_988_749_894_8;

    let Range {
        start: x_l,
        end: x_r,
    } = interval;

    if x_r - x_l < eps {
        let middle = (x_l + x_r) / 2.;
        return (middle, f(middle));
    }

    let x_1 = FRAC_1_PHI.mul_add(x_l - x_r, x_r);
    let x_2 = FRAC_1_PHI.mul_add(x_r - x_l, x_l);

    let i = match f(x_1).total_cmp(&f(x_2)) {
        Ordering::Less => x_1..x_r,
        Ordering::Equal => x_1..x_2,
        Ordering::Greater => x_l..x_2,
    };

    solve_for(i, eps)
}
