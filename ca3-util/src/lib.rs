use rand::distributions::{Distribution, Uniform};

pub fn rk4<F: Fn(f64) -> f64>(f: F, x: f64, dt: f64) -> f64 {
    let k1 = dt * f(x);
    let k2 = dt * f(x + k1 / 2.0);
    let k3 = dt * f(x + k2 / 2.0);
    let k4 = dt * f(x + k3);
    (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}

pub fn f_min(a: f64, b: f64) -> f64 {
    if a < b { a }
    else { b }
}

pub fn take_id(n_neuron: usize, n_connect: usize) -> Vec<usize> {
    if n_neuron < n_connect { panic!() }
    let ud = Uniform::new(0.0, 1.0);
    let mut ids: Vec<usize> = (0..n_neuron).collect();
    let mut ret: Vec<usize> = Vec::with_capacity(n_connect);

    for i in 0..n_connect {
        let n_rest = (n_neuron - i) as f64;
        let j = (ud.sample(&mut rand::thread_rng()) * n_rest) as usize;
        ret.push(ids.remove(j));
    }

    ret.sort();
    ret
}

pub fn judge(n: usize, p: f64) -> Vec<bool> {
    let mut ret = Vec::with_capacity(n);
    let ud = Uniform::new(0.0, 1.0);
    for _ in 0..n {
        if ud.sample(&mut rand::thread_rng()) < p {
            ret.push(true);
        } else {
            ret.push(false);
        }
    }

    ret
}