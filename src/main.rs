//
// Created by J. Blackburn - Feb 5 2025
//

mod perceptron;
use perceptron::{Data, Model};

fn main() {

    // TODO: Add collect training dat file 
 

        // Build model from data
    let data = Data::new("train.dat");
    let mut model = Model::new(data);

    model.print();
        // train the model via SGD
    model.fit();
    model.print();

    model.evaluate("test.dat"); 

        // predict target from user input
    // model.predict_from_xy();
}
