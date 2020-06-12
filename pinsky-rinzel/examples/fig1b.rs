use std::collections::HashMap;
use pinsky_rinzel::*;
use rayon::prelude::*;

fn main() {
    let mut params = HashMap::new();
    params.insert("n_neuron", 1.0);
    params.insert("n_connect", 0.0);

    let dt = 0.05;
    let mut neurons = Vec::new();
    let v_pre = vec![1.0];
    let gcs = vec![0.18, 0.6, 2.1, 3.6, 9.0, 15.0];
    let mut save = "time(ms)".to_string();

    for &g_c in gcs.iter() {
        params.insert("g_c", g_c);
        for i in 0..10 {
            neurons.push(Neuron::new(&params));
            save = save + ",g_c=" + &(g_c.to_string()) + "|clamp=" + &((i * 10).to_string());
        }
    }
    save = save + "\n";
    
    for t in 0..20*1100 { // 1.1 s
        neurons.par_iter_mut()
               .enumerate()
               .map(|(i, n)| {
                    n.soma.v = -10.0;
                    if t >= 20*1010 && t < 20*1013 {
                        n.soma.v = ((i % 10) * 10) as f64;
                    }
                    n.run(dt, &v_pre);
               })
               .collect::<Vec<()>>();
        if t >= 20*1000 {
            save = save + &((t as f64 * dt).to_string());
            for n in neurons.iter() {
                save = save + "," + &(n.dend.v.to_string());
            }
            save = save + "\n";
        }
    }

    println!("{}", save);
}