use ca3_util::*;

use crate::consts::*;

pub fn x_inf(alpha: f64, beta: f64) -> f64 {
    alpha / (alpha + beta)
}

pub fn tau_x(alpha: f64, beta: f64) -> f64 {
    1.0 / (alpha + beta)
}

pub fn kai(ca: f64) -> f64 {
    f_min(ca / 250.0, 1.0)
}

pub trait Channel {
    fn new(g: f64) -> Self;
    fn run(&mut self, v: f64, ca: f64, dt: f64);
    fn i_ion(&self, v: f64, ca: f64) -> f64;
}

pub struct NaChannel {
    pub g: f64,
    pub m: f64,
    pub h: f64,
}

impl Default for NaChannel {
    fn default() -> Self {
        Self::new(G_NA)
    }
}

impl Channel for NaChannel {
    fn new(g: f64) -> Self {
        let a = Self::alpha_m(V_S_REST);
        let b = Self::beta_m(V_S_REST);
        let m = x_inf(a, b);

        let a = Self::alpha_h(V_S_REST);
        let b = Self::beta_h(V_S_REST);
        let h = x_inf(a, b);

        Self { g, m, h, }
    }

    fn run(&mut self, v: f64, _ca: f64, dt: f64) {
        let a = Self::alpha_m(v);
        let b = Self::beta_m(v);
        self.m = x_inf(a, b);

        let a = Self::alpha_h(v);
        let b = Self::beta_h(v);
        let inf = x_inf(a, b);
        let tau = tau_x(a, b);
        let dx = |x: f64| (inf - x) / tau;
        self.h += rk4(dx, self.h, dt); 
    }

    fn i_ion(&self, v: f64, _ca: f64) -> f64 {
        self.g * self.m * self.m * self.h * (v - V_NA)
    }
}

impl NaChannel {
    pub fn alpha_m(v: f64) -> f64 {
        0.32 * (13.1 - v) / (((13.1 - v) / 4.0).exp() - 1.0)
    }

    pub fn beta_m(v: f64) -> f64 {
        0.28 * (v - 40.1) / (((v - 40.1) / 5.0).exp() - 1.0)
    }

    pub fn alpha_h(v: f64) -> f64 {
        0.128 * ((17.0 - v) / 18.0).exp()
    }

    pub fn beta_h(v: f64) -> f64 {
        4.0 / (1.0 + ((40.0 - v) / 5.0).exp())
    }
}

pub struct KdrChannel {
    pub g: f64,
    pub n: f64,
}

impl Default for KdrChannel {
    fn default() -> Self {
        Self::new(G_K_DR)
    }
}

impl Channel for KdrChannel {
    fn new(g: f64) -> Self {
        let a = Self::alpha_n(V_S_REST);
        let b = Self::beta_n(V_S_REST);
        let n = x_inf(a, b);

        Self { g, n, }
    }

    fn run(&mut self, v: f64, _ca: f64, dt: f64) {
        let a = Self::alpha_n(v);
        let b = Self::beta_n(v);
        let inf = x_inf(a, b);
        let tau = tau_x(a, b);
        let dx = |x: f64| (inf - x) / tau;
        self.n += rk4(dx, self.n, dt); 
    }

    fn i_ion(&self, v: f64, _ca: f64) -> f64 {
        self.g * self.n * (v - V_K)
    }
}

impl KdrChannel {
    pub fn alpha_n(v: f64) -> f64 {
        0.016 * (35.1 - v) / (((35.1 - v) / 5.0).exp() - 1.0)
    }

    pub fn beta_n(v: f64) -> f64 {
        0.25 * (0.5 - 0.025 * v).exp()
    }
}

pub struct CaChannel {
    pub g: f64,
    pub s: f64,
}

impl Default for CaChannel {
    fn default() -> Self {
        Self::new(G_CA)
    }
}

impl Channel for CaChannel {
    fn new(g: f64) -> Self {
        let a = Self::alpha_s(V_D_REST);
        let b = Self::beta_s(V_D_REST);
        let s = x_inf(a, b);

        Self { g, s, }
    }

    fn run(&mut self, v: f64, _ca: f64, dt: f64) {
        let a = Self::alpha_s(v);
        let b = Self::beta_s(v);
        let inf = x_inf(a, b);
        let tau = tau_x(a, b);
        let dx = |x: f64| (inf - x) / tau;
        self.s += rk4(dx, self.s, dt); 
    }

    fn i_ion(&self, v: f64, _ca: f64) -> f64 {
        self.g * self.s * self.s * (v - V_CA)
    }
}

impl CaChannel {
    pub fn alpha_s(v: f64) -> f64 {
        1.6 / (1.0 + (-0.072 * (v - 65.0)).exp())
    }

    pub fn beta_s(v: f64) -> f64 {
        0.02 * (v - 51.1) / (((v - 51.1) / 5.0).exp() - 1.0)
    }
}

pub struct KcChannel {
    pub g: f64,
    pub c: f64,
}

impl Default for KcChannel {
    fn default() -> Self {
        Self::new(G_K_C)
    }
}

impl Channel for KcChannel {
    fn new(g: f64) -> Self {
        let a = Self::alpha_c(V_D_REST);
        let b = Self::beta_c(V_D_REST);
        let c = x_inf(a, b);

        Self { g, c, }
    }

    fn run(&mut self, v: f64, _ca: f64, dt: f64) {
        let a = Self::alpha_c(v);
        let b = Self::beta_c(v);
        let inf = x_inf(a, b);
        let tau = tau_x(a, b);
        let dx = |x: f64| (inf - x) / tau;
        self.c += rk4(dx, self.c, dt); 
    }

    fn i_ion(&self, v: f64, ca: f64) -> f64 {
        self.g * self.c * kai(ca) * (v - V_K)
    }
}

impl KcChannel {
    pub fn alpha_c(v: f64) -> f64 {
        if v <= 50.0 {
            ((v - 10.0) / 11.0 - (v - 6.5) / 27.0).exp() / 18.975
        } else {
            2.0 * ((6.5 - v) / 27.0).exp()
        }
    }

    pub fn beta_c(v: f64) -> f64 {
        if v <= 50.0 {
            2.0 * ((6.5 - v) / 27.0).exp() - Self::alpha_c(v)
        } else {
            0.0
        }
    }
}

pub struct KahpChannel {
    pub g: f64,
    pub q: f64,
}

impl Default for KahpChannel {
    fn default() -> Self {
        Self::new(G_K_AHP)
    }
}

impl Channel for KahpChannel {
    fn new(g: f64) -> Self {
        let a = Self::alpha_q(CA_REST);
        let b = 0.001;
        let q = x_inf(a, b);

        Self { g, q, }
    }

    fn run(&mut self, _v: f64, ca: f64, dt: f64) {
        let a = Self::alpha_q(ca);
        let b = 0.001;
        let inf = x_inf(a, b);
        let tau = tau_x(a, b);
        let dx = |x: f64| (inf - x) / tau;
        self.q += rk4(dx, self.q, dt); 
    }

    fn i_ion(&self, v: f64, _ca: f64) -> f64 {
        self.g * self.q * (v - V_K)
    }
}

impl KahpChannel {
    pub fn alpha_q(ca: f64) -> f64 {
        f_min(ca * 0.2 * 1e-4, 0.01)
    }
}