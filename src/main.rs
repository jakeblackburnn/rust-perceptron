mod perceptron;
use perceptron::Data;

fn main() {
    perceptron::hello();

    let data = Data::new();

    data.print();
}
