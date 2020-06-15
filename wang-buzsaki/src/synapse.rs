use ca3_util::rk4;
use crate::consts::*;

pub struct Synapse {
    pub id: usize,

    pub g: f64,
    pub s: f64,

    pub alpha: f64,
    pub beta: f64,
    pub theta: f64,
}

impl Default for Synapse {
    fn default() -> Self {
        Self::new(0, G_SYN, ALPHA, BETA, THETA_SYN)
    }
}

impl Synapse {
    pub fn new(id: usize, g: f64, alpha: f64, beta: f64, theta: f64) -> Self {
        let s = 0.0;
        Self { id, g, s, alpha, beta, theta, }
    }

    pub fn run(&mut self, dt:f64, v_pre: &Vec<f64>) {
        let v = v_pre[self.id];
        let a = self.alpha;
        let b = self.beta;
        let ds = |s: f64| a * self.f(v) * (1.0 - s) - b * s;
        self.s += rk4(ds, self.s, dt);
    }

    pub fn i_syn_head(&self) -> f64 {
        self.g * self.s
    }

    pub fn f(&self, v: f64) -> f64 {
        1.0 / (1.0 + (-(v - self.theta) / 2.0))
    }
}