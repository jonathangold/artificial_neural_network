mod perceptron;
mod train;

fn main() {
    let data: Vec<(i32,i32)> = vec!((0,0),(0,1),(1,0),(1,1));
    let xorResult: Vec<i32> = vec!(0,1,1,0);
    
    let mut xorPerceptron = perceptron::newPerceptron(2);

    println!("{:?}", &xorPerceptron);
    train::train(&mut xorPerceptron, 10000, 0.1);
    println!("{:?}", &xorPerceptron);
}
