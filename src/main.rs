mod perceptron;
use perceptron::{Data, Model};

fn main() {
    let data = Data::new("train.dat");

    let mut model = Model::new(data);

    model.fit();

    model.predict_from_xy();
}
