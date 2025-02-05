mod perceptron;
use perceptron::{Data, Model};

fn main() {
    let data = Data::new("train.dat");
    data.print();

    let mut model = Model::new(data);
    
    model.print();

    model.fit();

    model.print();
}
