use tailcall::tailcall;

fn f(x: f64) -> f64 {
    -x.powi(2) + 4_f64.mul_add(x, 1.)
}

fn f_prime(x: f64) -> f64 {
    (-2_f64).mul_add(x, 4.)
}

#[allow(unreachable_code)]
#[tailcall]
pub fn solve_for(x_0: f64, alpha: f64, n: usize) -> (f64, f64) {
    if n == 0 {
        return (x_0, f(x_0));
    }

    solve_for(alpha.mul_add(f_prime(x_0), x_0), alpha, n - 1)
}
