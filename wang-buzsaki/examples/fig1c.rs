use std::collections::HashMap;
use wang_buzsaki::neuron::*;
use wang_buzsaki::consts::E_SYN;

fn main() {
    let mut params = HashMap::new();
    let mut v_pre = vec![0.0, 0.0];
    let dt = 0.05;  // ms

    let mut neurons = Vec::new();
    let mut save = "time(ms),V_pre(mV),I_app(uA/cm2),V_post(mV),I_syn(uA/cm2)\n".to_string();
    params.insert("n", 1.0);
    params.insert("m_syn", 0.0);
    neurons.push(InterNeuron::new(&params));
    params.insert("m_syn", 1.0);
    neurons.push(InterNeuron::new(&params));

    for t in 0..20*1150 {
        if t == 20*1012 { neurons[0].i_app = 2.0; }
        if t == 20*1025 { neurons[0].i_app = 0.0; }
        v_pre = neurons.iter_mut()
                       .map(|n| n.run(dt, &v_pre))
                       .collect::<Vec<f64>>();
        if t >= 20*1000 {
            save = save
                    + &(((t - 20*1000) as f64 * dt).to_string())
                    + "," + &(neurons[0].v.to_string())
                    + "," + &(neurons[0].i_app.to_string())
                    + "," + &(neurons[1].v.to_string())
                    + "," + &((neurons[1].synapses[0].i_syn_head() * (neurons[1].v - E_SYN)).to_string())
                    + "\n";
        }
    }

    println!("{}", save);
}