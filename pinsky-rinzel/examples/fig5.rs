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
    let mut somas = Vec::new();
    let mut dends = Vec::new();
    let mut neurons = Vec::new();
    let mut neurons_inf = Vec::new();
    let v_pre = vec![1.0];
    let soma_i_vec = vec![-0.175, -0.165, -0.125, 0.0, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5, 2.75, 3.0];
    let dend_i_vec = vec![0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5, 2.75, 3.0];
    let i_vec = vec![-0.25, -0.125, 0.0, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5, 2.75, 3.0];

    let mut save = "time(ms)".to_string();
    for &i in soma_i_vec.iter() {
        params.insert("i_s", i);
        somas.push(Soma::new(&params));
        save = save + ",soma" + &(i.to_string());
    }

    params.insert("i_s", 0.0);
    params.insert("g_c", 0.0);
    for &i in dend_i_vec.iter() {
        params.insert("i_d", i);
        dends.push(Dend::new(&params));
        save = save + ",dend" + &(i.to_string());
    }

    params.insert("i_s", 0.0);
    params.insert("i_d", 0.0);
    params.insert("g_c", 2.1);
    for &i in i_vec.iter() {
        params.insert("i_s", i);
        neurons.push(Neuron::new(&params));
        save = save + ",g_c21i_s" + &(i.to_string());
    }

    params.insert("i_s", 0.0);
    params.insert("i_d", 0.0);
    params.insert("g_c", 2.1);
    for &i in i_vec.iter() {
        params.insert("i_d", i);
        neurons.push(Neuron::new(&params));
        save = save + ",g_c21i_d" + &(i.to_string());
    }

    params.insert("i_s", 0.0);
    params.insert("i_d", 0.0);
    params.insert("g_c", 0.0);
    for &i in i_vec.iter() {
        params.insert("i_s", i);
        neurons_inf.push(Neuron::new(&params));
        save = save + ",g_cinf" + &(i.to_string());
    }

    save = save + "\n";
    for i in 0..20*1000*11 { // 11.0 s
        somas.par_iter_mut().map(|n| n.run(0.0, dt, &v_pre)).collect::<Vec<()>>();
        dends.par_iter_mut().map(|n| n.run(0.0, dt, &v_pre)).collect::<Vec<()>>();
        neurons.par_iter_mut().map(|n| {n.run(dt, &v_pre);}).collect::<Vec<()>>();
        neurons_inf.par_iter_mut().map(|n| {
                                        n.run(dt, &v_pre);
                                        let mv = (n.soma.v + n.dend.v) / 2.0;
                                        n.soma.v = mv;
                                        n.dend.v = mv;
                                    }).collect::<Vec<()>>();
        if i >= 20*1000 {
            save = save + &((i as f64 * dt).to_string());
            for n in somas.iter() {
                save = save + "," + &(n.v.to_string())
            }
            for n in dends.iter() {
                save = save + "," + &(n.v.to_string())
            }
            for n in neurons.iter() {
                save = save + "," + &(n.soma.v.to_string())
            }
            for n in neurons_inf.iter() {
                save = save + "," + &(n.soma.v.to_string())
            }
            save = save + "\n";
        }
    }
    
    println!("{}", save);
}