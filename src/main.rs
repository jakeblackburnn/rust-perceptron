mod perceptron;
use perceptron::{Data, Model};

fn main() {
    let data = Data::new("train.dat");
    data.print();

    let model = Model::new(data);
    model.print();
}
