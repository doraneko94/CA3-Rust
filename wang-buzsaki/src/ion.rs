use ca3_util::rk4;
use crate::consts::*;

fn x_inf(a: f64, b: f64) -> f64 {
    a / (a + b)
}

pub trait Channel {
    fn new(g: f64, v: f64) -> Self;
    fn run(&mut self, v: f64, dt: f64);
    fn i_ion(&self, v: f64) -> f64;
}

pub struct NaChannel {
    pub g: f64,
    pub m: f64,
    pub h: f64,
}

impl Default for NaChannel {
    fn default() -> Self {
        Self::new(G_NA, V_REST)
    }
}

impl Channel for NaChannel {
    fn new(g: f64, v: f64) -> Self {
        let a = Self::alpha_m(v);
        let b = Self::beta_m(v);
        let m = x_inf(a, b);

        let a = Self::alpha_h(v);
        let b = Self::beta_h(v);
        let h = x_inf(a, b);

        Self { g, m, h, }
    }

    fn run(&mut self, v: f64, dt: f64) {
        let a = Self::alpha_m(v);
        let b = Self::beta_m(v);
        self.m = x_inf(a, b);

        let a = Self::alpha_h(v);
        let b = Self::beta_h(v);
        let dx = |x: f64| PHI * (a * (1.0 - x) - b * x);
        self.h += rk4(dx, self.h, dt); 
    }

    fn i_ion(&self, v: f64) -> f64 {
        self.g * self.m.powi(3) * self.h * (v - E_NA)
    }
}

impl NaChannel {
    pub fn alpha_m(v: f64) -> f64 {
        -0.1 * (v + 35.0) / ((-0.1 * (v + 35.0)).exp() - 1.0)
    }

    pub fn beta_m(v: f64) -> f64 {
        4.0 * (-(v + 60.0) / 18.0).exp()
    }

    pub fn alpha_h(v: f64) -> f64 {
        0.07 * (-(v + 58.0) / 20.0).exp()
    }

    pub fn beta_h(v: f64) -> f64 {
        1.0 / ((-0.1 * (v + 28.0)).exp() + 1.0)
    }
}

pub struct KChannel {
    pub g: f64,
    pub n: f64,
}

impl Default for KChannel {
    fn default() -> Self {
        Self::new(G_K, V_REST)
    }
}

impl Channel for KChannel {
    fn new(g: f64, v: f64) -> Self {
        let a = Self::alpha_n(v);
        let b = Self::beta_n(v);
        let n = x_inf(a, b);

        Self { g, n, }
    }

    fn run(&mut self, v: f64, dt: f64) {
        let a = Self::alpha_n(v);
        let b = Self::beta_n(v);
        let dx = |x: f64| PHI * (a * (1.0 - x) - b * x);
        self.n += rk4(dx, self.n, dt); 
    }

    fn i_ion(&self, v: f64) -> f64 {
        self.g * self.n.powi(4) * (v - E_K)
    }
}

impl KChannel {
    pub fn alpha_n(v: f64) -> f64 {
        -0.01 * (v + 34.0) / ((-0.1 * (v + 34.0)).exp() - 1.0)
    }

    pub fn beta_n(v: f64) -> f64 {
        0.125 * (-(v + 44.0) / 80.0).exp()
    }
}