mod bayesian_network;
use std::collections::HashMap;

use bayesian_network::BayesianNetwork;
mod cpt;
use cpt::CPT;

fn main() {
    let mut net = BayesianNetwork::new();
    net.add_node("A", vec![("T", 0.6), ("F", 0.4)]);
    net.add_node("B", vec![("T", 0.5), ("F", 0.5)]);
    net.display_node("A");
    net.display_node("B");
    net.add_parents("B", vec!["A"]);
}
