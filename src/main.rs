extern crate rand;

mod perceptron;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    }

fn f(x:i32) -> i32 {
    let mut a:i32 = rand::thread_rng().gen_range(-5, 6);
    let mut b:i32 = rand::thread_rng().gen_range(-50, 51);
    a * x + b
}

fn isAboveLine(point:&Vec<i32>, f:&Fn(i32) -> i32) -> i32 {
    let x = point[0];
    let y = point[1];
    match x.cmp(&f(x)) {
        Ordering::Greater => {1}
        _ => {0}
    }
}

fn train(p: perceptron::Perceptron, iters: i32, rate: f32) {
    for i in 1..iters {
        let mut point: Vec<i32> = Vec::new();
        point.push(randInt());
        point.push(randInt());
        let x = point[0];
        let actual = p.process(&point);
        let expected = isAboveLine(&point, &f);
        let delta = expected - actual;                           
    }
}

fn randInt() -> i32 {
    rand::thread_rng().gen_range(-100, 101)   
}
