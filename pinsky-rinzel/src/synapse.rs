use ca3_util::*;

use crate::consts::*;

pub fn h_func(x: f64, threshold: f64) -> f64 {
    if x >= threshold { 1.0 }
    else { 0.0 }
}

pub struct Synapse {
    pub g_nmda: f64,
    pub g_ampa: f64,
    pub s: f64,
    pub w: f64,
    pub connect_id: Vec<usize>,
}

impl Default for Synapse {
    fn default() -> Self {
        Self::new(G_NMDA, G_AMPA, N_NEURON, N_CONNECT)
    }
}

impl Synapse {
    pub fn new(g_nmda: f64, g_ampa: f64, n_neuron: usize, n_connect: usize) -> Self {
        let (s, w) = (0.0, 0.0);
        let connect_id = take_id(n_neuron, n_connect);
        Self { g_nmda, g_ampa, s, w, connect_id, }
    }

    pub fn run(&mut self, dt: f64, v_pre: &Vec<f64>) {
        let h_sum: f64 = self.connect_id.iter().map(|&i| h_func(v_pre[i], 10.0)).sum();
        let ds = |s: f64| h_sum - s / 150.0;
        self.s += rk4(ds, self.s, dt);
        if self.s > S_MAX { self.s = S_MAX }

        let h_sum: f64 = self.connect_id.iter().map(|&i| h_func(v_pre[i], 20.0)).sum();
        let dw = |w: f64| h_sum - w / 2.0;
        self.w += rk4(dw, self.w, dt);
    }

    pub fn i_syn(&self, v: f64) -> f64 {
        (self.g_nmda * self.s / (1.0 + 0.28 * (-0.062 * (v - 60.0)).exp()) + self.g_ampa * self.w) * (v - V_SYN)
    }
}