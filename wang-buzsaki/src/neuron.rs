use std::collections::HashMap;
use ca3_util::{rk4, judge};

use crate::consts::*;
use crate::ion::*;
use crate::synapse::*;

pub struct InterNeuron {
    pub v: f64,
    pub i_app: f64,

    pub na: NaChannel,
    pub k: KChannel,
    pub synapses: Vec<Synapse>,

    pub n_syn: usize,
}

impl InterNeuron {
    pub fn new(params: &HashMap<&str, f64>) -> Self {
        let mut v = V_REST;
        let mut i_app = 0.0;
        let mut g_na = G_NA;
        let mut g_k = G_K;
        let mut g_syn = G_SYN;
        let mut alpha = ALPHA;
        let mut beta = BETA;
        let mut theta = THETA_SYN;
        let mut n = N as f64;
        let mut m_syn = M_SYN as f64;

        for (&key, &val) in params.iter() {
            match key {
                "v" => v = val,
                "i_app" => i_app = val,
                "g_na" => g_na = val,
                "g_k" => g_k = val,
                "g_syn" => g_syn = val,
                "alpha" => alpha = val,
                "beta" => beta = val,
                "theta" => theta = val,
                "n" => n = val,
                "m_syn" => m_syn = val,
                _ => {}
            };
        }

        let na = NaChannel::new(g_na, v);
        let k = KChannel::new(g_k, v);

        let p = if n == m_syn {
            1.0
        } else if n < m_syn {
            panic!("m_syn is larger than n!")
        } else {
            m_syn / n
        };

        let mut synapses = Vec::new();
        let v_judge = judge(n as usize, p);
        let mut n_syn = 0;
        for (i, &b) in v_judge.iter().enumerate() {
            if b { 
                synapses.push(Synapse::new(i, g_syn, alpha, beta, theta));
                n_syn += 1;
            }
        }
        if n_syn == 0 { n_syn = 1; }

        Self { v, i_app, na, k, synapses, n_syn, }
    }

    pub fn run(&mut self, dt: f64, v_pre: &Vec<f64>) -> f64 {
        self.na.run(self.v, dt);
        self.k.run(self.v, dt);
        let _ = self.synapses.iter_mut()
                             .map(|s| s.run(dt, v_pre))
                             .collect::<Vec<()>>();

        let i_syn_h = self.synapses.iter_mut()
                                   .map(|s| s.i_syn_head())
                                   .sum::<f64>();
        let n_syn = self.n_syn as f64;
        let i_syn = |v: f64| i_syn_h * (v - E_SYN) / n_syn;
        let i_app = self.i_app;
        let dv = |v: f64| (- self.na.i_ion(v) - self.k.i_ion(v) - G_L * (v - E_L) - i_syn(v) + i_app) / C_M;
        self.v += rk4(dv, self.v, dt);
        
        self.v
    }
}