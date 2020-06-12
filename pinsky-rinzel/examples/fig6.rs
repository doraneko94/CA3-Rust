use std::collections::HashMap;
use pinsky_rinzel::*;
use rayon::prelude::*;

fn main() {
    let mut params = HashMap::new();
    params.insert("n_neuron", 1.0);
    params.insert("n_connect", 0.0);
    params.insert("i_s", 0.0);
    params.insert("i_d", 0.0);
    params.insert("g_c", 0.0);

    let dt = 0.05;
    let mut neurons = Vec::new();
    let v_pre = vec![1.0];
    let i_vec = vec![-0.25, -0.125, 0.0, 0.125, 0.25, 0.375, 0.5, 0.625, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7];

    let mut save = "time(ms)".to_string();

    params.insert("g_c", 1.85);
    for &i in i_vec.iter() {
        params.insert("i_s", i);
        neurons.push(Neuron::new(&params));
        save = save + ",i_s" + &(i.to_string());
    }

    save = save + "\n";
    for i in 0..20*1000*11 { // 11.0 s
        neurons.par_iter_mut().map(|n| {n.run(dt, &v_pre);}).collect::<Vec<()>>();
        if i >= 20*1000 {
            save = save + &((i as f64 * dt).to_string());
            for n in neurons.iter() {
                save = save + "," + &(n.soma.v.to_string())
            }
            save = save + "\n";
        }
    }
    
    println!("{}", save);
}