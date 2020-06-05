pub fn rk4<F: Fn(f64) -> f64>(f: F, x: f64, dt: f64) -> f64 {
    let k1 = dt * f(x);
    let k2 = dt * f(x + k1 / 2.0);
    let k3 = dt * f(x + k2 / 2.0);
    let k4 = dt * f(x + k3);
    (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}