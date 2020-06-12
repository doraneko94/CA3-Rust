use std::collections::HashMap;

use ca3_util::*;

use crate::consts::*;
use crate::ion::*;
use crate::synapse::*;

pub trait Compartment {
    fn new(params: &HashMap<&str, f64>) -> Self;
    fn run(&mut self, v_opp: f64, dt: f64, v_pre: &Vec<f64>);
}

pub struct Soma {
    pub v: f64,
    pub g_c: f64,
    pub p: f64,

    pub na: NaChannel,
    pub k_dr: KdrChannel,

    pub i_s: f64,
}

impl Default for Soma {
    fn default() -> Self {
        Self::new(&HashMap::new())
    }
}

impl Compartment for Soma {
    fn new(params: &HashMap<&str, f64>) -> Self {
        let v = V_S_REST;
        let mut g_c = G_C;
        let mut p = P;

        let mut g_na = G_NA;
        let mut g_k_dr = G_K_DR;

        let mut i_s = I_S;
        for (&key, &val) in params.iter() {
            match key {
                "g_c" => g_c = val,
                "p" => p = val,
                "g_na" => g_na = val,
                "g_k_dr" => g_k_dr = val,
                "i_s" => i_s = val,
                _ => {}
            };
        }

        let na = NaChannel::new(g_na);
        let k_dr = KdrChannel::new(g_k_dr);

        Self { v, g_c, p, na, k_dr, i_s, }
    }

    fn run(&mut self, v_opp: f64, dt: f64, _v_pre: &Vec<f64>) {
        self.na.run(self.v, CA_REST, dt);
        self.k_dr.run(self.v, CA_REST, dt);

        let g_c = self.g_c;
        let p = self.p;
        let i_s = self.i_s;
        let dv = |v: f64| (- G_L * (v - V_L)
                           - self.na.i_ion(v, CA_REST) - self.k_dr.i_ion(v, CA_REST)
                           + (g_c * (v_opp - v) + i_s) / p)
                          / C_M;
        self.v += rk4(dv, self.v, dt);
    }
}

pub struct Dend {
    pub v: f64,
    pub g_c: f64,
    pub omp: f64, // 1 - p (one minus p)
    pub ca_i : f64,

    pub ca: CaChannel,
    pub k_ahp: KahpChannel,
    pub k_c: KcChannel,

    pub syn: Synapse,

    pub i_d: f64,
}

impl Default for Dend {
    fn default() -> Self {
        Self::new(&HashMap::new())
    }
}

impl Compartment for Dend {
    fn new(params: &HashMap<&str, f64>) -> Self {
        let v = V_D_REST;
        let mut g_c = G_C;
        let mut omp = 1.0 - P;
        let mut ca_i = CA_REST;

        let mut g_ca = G_CA;
        let mut g_k_ahp = G_K_AHP;
        let mut g_k_c = G_K_C;

        let mut g_nmda = G_NMDA;
        let mut g_ampa = G_AMPA;
        let mut n_neuron = N_NEURON;
        let mut n_connect = N_CONNECT;

        let mut i_d = I_D;
        for (&key, &val) in params.iter() {
            match key {
                "g_c" => g_c = val,
                "p" => omp = 1.0 - val,
                "ca_i" => ca_i = val,
                "g_ca" => g_ca = val,
                "g_k_ahp" => g_k_ahp = val,
                "g_k_c" => g_k_c = val,
                "g_nmda" => g_nmda = val,
                "g_ampa" => g_ampa = val,
                "n_neuron" => n_neuron = val as usize,
                "n_connect" => n_connect = val as usize,
                "i_d" => i_d = val,
                _ => {}
            };
        }

        let ca = CaChannel::new(g_ca);
        let k_ahp = KahpChannel::new(g_k_ahp);
        let k_c = KcChannel::new(g_k_c);
        let syn = Synapse::new(g_nmda, g_ampa, n_neuron, n_connect);

        Self { v, g_c, omp, ca_i, ca, k_ahp, k_c, syn, i_d, }
    }

    fn run(&mut self, v_opp: f64, dt: f64, v_pre: &Vec<f64>) {
        self.ca.run(self.v, self.ca_i, dt);
        self.k_ahp.run(self.v, self.ca_i, dt);
        self.k_c.run(self.v, self.ca_i, dt);
        self.syn.run(dt, v_pre);

        let g_c = self.g_c;
        let omp = self.omp;
        let i_d = self.i_d;
        let ca_i = self.ca_i;
        let i_ca = self.ca.i_ion(self.v, self.ca_i);
        let dv = |v: f64| (- G_L * (v - V_L)
                           - self.ca.i_ion(v, ca_i) - self.k_ahp.i_ion(v, ca_i) - self.k_c.i_ion(v, ca_i)
                           + (-self.syn.i_syn(v) + g_c * (v_opp - v) + i_d) / omp)
                          / C_M;
        self.v += rk4(dv, self.v, dt);

        let dca = |ca: f64| -0.13 * i_ca - 0.075 * ca;
        self.ca_i += rk4(dca, self.ca_i, dt);
    }
}