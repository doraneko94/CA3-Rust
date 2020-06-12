use std::collections::HashMap;
use pinsky_rinzel::*;
use rayon::prelude::*;

fn main() {
    let mut neurons = Vec::new();
    let dt = 0.05;
    let mut v_pre = vec![50.0];

    let mut params = HashMap::new();
    params.insert("n_neuron", 1.0);
    params.insert("n_connect", 1.0);

    // A
    params.insert("i_s", 0.75);
    params.insert("g_nmda", 0.0);
    params.insert("g_c", 2.1);
    neurons.push(Neuron::new(&params));

    // B
    params.insert("i_s", -0.5);
    params.insert("g_nmda", 1.25);
    params.insert("g_c", 2.1);
    neurons.push(Neuron::new(&params));

    // C
    params.insert("i_s", 2.5);
    params.insert("g_nmda", 0.0);
    params.insert("g_c", 2.1);
    neurons.push(Neuron::new(&params));

    // D
    params.insert("i_s", 2.5);
    params.insert("g_nmda", 0.0);
    params.insert("g_c", 10.5);
    neurons.push(Neuron::new(&params));

    // E
    params.insert("i_s", -0.5);
    params.insert("g_nmda", 1.75);
    params.insert("g_c", 1.425);
    neurons.push(Neuron::new(&params));
    //neurons[4].dend.k_ahp.q = 0.4;

    let mut save = "time(ms),A|V_s(mV),A|Ca,A|q,B|V_s(mV),B|Ca,B|q,C|V_s(mV),C|Ca,C|q,D|V_s(mV),D|Ca,D|q,E|V_s(mV),E|Ca,E|q\n".to_string();
    for t in 0..20*1600 { // 1.6 s
        if t % 250 == 0 { v_pre[0] = 50.0; }
        else { v_pre[0] = 0.0; }
        neurons.par_iter_mut().map(|n| {n.run(dt, &v_pre);}).collect::<Vec<()>>();
        save = save + &((t as f64 * dt).to_string());
        for i in 0..5 {
            save = save
                    + "," + &(neurons[i].soma.v.to_string()) 
                    + "," + &(neurons[i].dend.ca_i.to_string()) 
                    + "," + &(neurons[i].dend.k_ahp.q.to_string());
        }
        save = save + "\n";
    }
    
    println!("{}", save);
}