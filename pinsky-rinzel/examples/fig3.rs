use std::collections::HashMap;
use pinsky_rinzel::*;

fn main() {
    let dt = 0.05;
    let v_pre = vec![0.0];

    let mut params = HashMap::new();
    params.insert("n_neuron", 1.0);
    params.insert("n_connect", 0.0);
    params.insert("i_s", 0.75);
    params.insert("g_nmda", 0.0);
    params.insert("g_c", 2.1);

    let mut neuron = Neuron::new(&params);

    let mut save = "time(ms),V_s(mV),V_d(mV),Ca\n".to_string();
    for t in 0..20*1600 { // 1.6 s
        neuron.run(dt, &v_pre);
        save = save + &((t as f64 * dt).to_string())
                    + "," + &(neuron.soma.v.to_string()) 
                    + "," + &(neuron.dend.v.to_string()) 
                    + "," + &(neuron.dend.ca_i.to_string())
                    + "\n";
    }
    
    println!("{}", save);
}