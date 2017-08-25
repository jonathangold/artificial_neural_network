extern crate rand;

use std::cmp::Ordering;

use rand::Rng;

fn main() {
    let hstest = heavyside(-0.232);
    println!("{}", hstest);
    let rntest = randomNumber();
    println!("{}", rntest);
    let perctest = newPerceptron(5);
    println!("{:?}", perctest);
}

#[derive(Debug)]
struct Perceptron {
    weights: Vec<f32>,
    bias: f32
}

fn heavyside(value: f32) -> i32 {
    let zero = 0.0f32;
    match value.partial_cmp(&zero).unwrap() {
        Ordering::Less => {0},
        Ordering::Equal => {0},
        Ordering::Greater => {1}
    }
}

fn newPerceptron(inputs: i32) -> Perceptron {
    let mut randomWeights: Vec<f32> = Vec::new();
    for i in 0..inputs {
        randomWeights.push(randomNumber());
    }
    Perceptron { weights: randomWeights, bias: randomNumber() }
}

fn randomNumber() -> f32 {
    let unscaled = rand::thread_rng().gen_range(-10000, 10001);
    (unscaled as f32) / 10000.0
}