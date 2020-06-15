use std::collections::HashMap;
use wang_buzsaki::neuron::*;

fn main() {
    let mut params = HashMap::new();
    params.insert("n", 1.0);
    params.insert("m_syn", 0.0);
    params.insert("i_app", 2.0);

    let mut n = InterNeuron::new(&params);
    let dt = 0.05;  // ms
    let v_pre = vec![0.0];

    for _ in 0..20*10 {
        println!("{}", n.run(dt, &v_pre));
    }  
}