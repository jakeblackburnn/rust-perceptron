//
// Created by J. Blackburn - Feb 5 2025
//

mod perceptron;
use perceptron::{Data, Model};

fn main() {

    // TODO: Add collect training dat file 
 

        // Build model from data
    let data = Data::verbose_new("train.dat");
    let mut model = Model::new(data);

    model.print();
        // train the model via SGD
    model.verbose_fit();
    model.print();

    model.verbose_evaluate("test.dat");



        // evaluate the model repeatedly, calculate average accuracy
    evaluate_average_accuracy("train.dat", "test.dat"); 

        // predict target from user input
    // model.predict_from_xy();
}



pub fn evaluate_average_accuracy(train_file: &str, test_file: &str) {

    println!("");
    println!("========== EVALUATING AVERAGE PERFORMANCE ==========");

    let mut total_accuracy: f64 = 0.0;
    let mut runs: u32 = 0;

    while (total_accuracy < 10000000.0) {

        let training_data  = Data::new(train_file);
        let mut model      = Model::new(training_data);
        model.fit();

        total_accuracy += model.evaluate(test_file); 
        runs  += 1;

    }

    let avg_accuracy = total_accuracy / runs as f64;

    println!("");
    println!("Average Accuracy: {:.3}% in {} runs", avg_accuracy, runs);

}

