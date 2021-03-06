use std::collections::HashMap;
use rayon::prelude::*;
use rand::distributions::{Distribution, Normal};
use wang_buzsaki::neuron::*;

fn main() {
    let mut params = HashMap::new();
    params.insert("n", 1.0);
    params.insert("m_syn", 0.0);
    let v_pre = vec![0.0];
    let dt = 0.05;  // ms

    let mut neurons = Vec::new();
    let mut save = "time(ms)".to_string();
    let nd = Normal::new(0.0, 0.03);
    
    for i in 1..11 {
        for j in 0..100 {
            save = save + "," + &(i.to_string()) + "_" + &(j.to_string()) + "_V(mV)";
            let i_app = i as f64 / 10.0 + nd.sample(&mut rand::thread_rng());
            params.insert("i_app", i_app);
            neurons.push(InterNeuron::new(&params));
        }
    }
    save = save + "\n";

    for t in 0..20*2000 {
        let v_vec = neurons.par_iter_mut()
                           .map(|n| n.run(dt, &v_pre))
                           .collect::<Vec<f64>>();
        if t >= 20*1000 {
            save = save + &((t as f64 * dt).to_string());
            for &v in v_vec.iter() {
                save = save + "," + &(v.to_string());
            }
            save = save + "\n";
        }
    }

    println!("{}", save);
}