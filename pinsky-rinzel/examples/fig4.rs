use std::collections::HashMap;
use pinsky_rinzel::*;
use rayon::prelude::*;

fn main() {
    let mut neurons = Vec::new();
    let dt = 0.05;
    let v_pre = vec![0.0];

    let mut params = HashMap::new();
    params.insert("n_neuron", 1.0);
    params.insert("n_connect", 0.0);

    let mut save = "time(ms)".to_string();
    for i in 0..11 {
        params.insert("i_s", (i+1) as f64 * 0.25);
        neurons.push(Neuron::new(&params));
        save = save + "," + &(i.to_string()) + "|V_s"
                    + "," + &(i.to_string()) + "|Ca";
    }
    save = save + "\n";

    for t in 0..20*1000*3 { // 3.0 s
        neurons.par_iter_mut().map(|n| {n.run(dt, &v_pre);}).collect::<Vec<()>>();
        save = save + &((t as f64 * dt).to_string());
        for i in 0..11 {
            save = save
                    + "," + &(neurons[i].soma.v.to_string()) 
                    + "," + &(neurons[i].dend.ca_i.to_string());
        }
        save = save + "\n";
    }
    
    println!("{}", save);
}