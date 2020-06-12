use std::collections::HashMap;

use crate::compartment::*;

pub struct Neuron {
    pub soma: Soma,
    pub dend: Dend,
}

impl Default for Neuron {
    fn default() -> Self {
        Self::new(&HashMap::new())
    }
}

impl Neuron {
    pub fn new(params: &HashMap<&str, f64>) -> Self {
        let soma = Soma::new(params);
        let dend = Dend::new(params);
        Self { soma, dend, }
    }

    pub fn run(&mut self, dt: f64, v_pre: &Vec<f64>) -> f64 {
        let v_s = self.soma.v;
        let v_d = self.dend.v;
        self.soma.run(v_d, dt, v_pre);
        self.dend.run(v_s, dt, v_pre);

        self.soma.v
    }
}