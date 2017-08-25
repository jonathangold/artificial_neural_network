extern crate rand;

use std::cmp::Ordering;

use self::rand::Rng;

#[derive(Debug)]
pub struct Perceptron {
    weights: Vec<f32>,
    bias: f32
}

impl Perceptron {
    pub fn process(&self, inputs:Vec<i32>) -> i32 {
        let mut sum = self.bias;
        for i in 0..inputs.len() {
            sum = (inputs[i] as f32) * &self.weights[i];    
        }
        heavyside(sum)
    }
    pub fn adjust(&mut self, inputs:Vec<i32>, delta:i32, learningRate:f32) {
        for i in 0..inputs.len(){
            self.weights[i] += (inputs[i] as f32) * (delta as f32) * learningRate;
        }
        self.bias += (delta as f32) * learningRate;
    }
}

pub fn newPerceptron(inputs: i32) -> Perceptron {
    let mut randomWeights: Vec<f32> = Vec::new();
    for i in 0..inputs {
        randomWeights.push(randomNumber());
    }
    Perceptron { weights: randomWeights, bias: randomNumber() }
}

fn heavyside(value: f32) -> i32 {
    let zero = 0.0f32;
    match value.partial_cmp(&zero).unwrap() {
        Ordering::Less => {0},
        Ordering::Equal => {0},
        Ordering::Greater => {1}
    }
}

fn randomNumber() -> f32 {
    let unscaled = rand::thread_rng().gen_range(-10000, 10001);
    (unscaled as f32) / 10000.0
}

