use std::{collections::HashMap, fmt::Error};
use crate::cpt::CPT;

pub struct BayesianNetwork {
    // Each node has a name and a conditional probability table: (name, CPT)
    nodes: HashMap<&'static str, CPT>,

    // Directed edge from parent to child: (parent, child)
    edges: HashMap<&'static str, Vec<&'static str>>,
}

impl BayesianNetwork {

    pub fn new() -> Self {
        BayesianNetwork {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_parents(&mut self, child: &'static str, parents: Vec<&'static str>) -> () {
        /*
        Set parent to child.

        Args:
            child (&str): The identifier (name) of the child node.
            parent (&str): The identifier (name) of the parent node.

        Returns:
            None
        */
        for parent in parents.into_iter() {
            self.edges.entry(child).or_insert_with(Vec::new).push(parent);
        }
    }

    pub fn add_node(&mut self, name: &'static str, probabilities: Vec<(&str, f32)>) -> () {
        /*
        Add a node to the Bayesian network.

        Args:
            name (str): The name (identifier) of the node.
            probabilities (Vec<(&str, f32)>): All possible values that this node can take and their corresponding probabilities.

        NOTE: Sum of probabilities must equal 1.

        Returns:
            None
        */

        if self.nodes.contains_key(name) {
            panic!("Node `{}` already exists in the network!", name);
        }

        let mut cpt: HashMap<String, f32> = HashMap::new();
        for (n, p) in probabilities {
            cpt.insert(n.to_string(), p);
        }
        self.nodes.insert(name, CPT::new(cpt));

    }

    pub fn display_node(&mut self, name: &str) -> () {
        /*
        Print the possible values and their corresponding values.

        Args:
            name (&str): The name (identifier) of the node.

        Returns:
            None
        */
        println!("Values: {}", self.nodes.get(name).unwrap());
    }

}