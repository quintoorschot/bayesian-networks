use core::fmt;
use std::collections::HashMap;

// Set tolerance of floating point arithmetic imprecision
const EPSILON: f32 = 1e-6;

pub struct CPT {
    probabilities: HashMap<String, f32>
}

impl CPT {

    pub fn new(probabilities: HashMap<String, f32>) -> Self {

        // Allow for floating point arithmetic imprecision
        if (probabilities.values().map(|&v| v).sum::<f32>() - 1.).abs() > EPSILON {
            panic!("Sum of probabilities in CPT must equal 1!");
        }

        CPT { probabilities: probabilities }
    }

}

impl fmt::Display for CPT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.probabilities)
    }
}