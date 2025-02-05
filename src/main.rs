mod perceptron;
use perceptron::Data;

fn main() {
    let data = Data::new("train.dat");

    data.print();
}
