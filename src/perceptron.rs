//
// Created by J. Blackburn - Feb 5 2025
//

use std::fs;
use std::io;
use rand::Rng;

    // data struct - stores sets of training / testing examples
    // TODO: rename data fields
pub struct Data {
    elements: Vec<Vec<f64>>,
    targets: Vec<i32>,
    rows: usize,
    columns: usize,
}

impl Data {

        // populate data struct with messages
    pub fn verbose_new(filename: &str) -> Self {

        println!("");
        println!("========== LOADING DATA ==========");

        let data = Data::new(filename);

        println!("");
        println!("Successfully loaded data from file: {}! \n", filename);

        data
    }
    
    
        // populate data struct silently 
    pub fn new(filename: &str) -> Self {

        let content = fs::read_to_string(filename)
                    .expect("Failed to read data file");
        let lines = content.lines();

        let mut elements: Vec<Vec<f64>> = Vec::new();
        let mut targets: Vec<i32> = Vec::new();

        for line in lines {

            let mut items = line.split(" ")
                            .map(|s| s.parse::<f64>().unwrap())
                            .collect::<Vec<f64>>();

            targets.push(
                items.pop()
                   .unwrap()
                   .round() 
                   as i32
            );

            elements.push(items);
        }

        let rows = elements.len();
        let columns = elements[0].len();

            // return populated data struct
        Data {
            elements,
            targets,
            rows,
            columns
        }
    }

        // TODO: rename print method

    pub fn print(&self) {

        println!("");
        println!("========== DISPLAYING DATA ==========\n");

        let mut i = 0;
        for feature_vector in &self.elements {
            print!("{:?}", feature_vector);
            println!("  {}", self.targets[i]);
            i += 1;
        }
        println!("\n");
    }

}





    // model struct 
pub struct Model {
    data: Data,
    weights: Vec<f64>,
}

impl Model {

        // populate model from data struct and random weights
    pub fn new(data: Data) -> Self {

        let length = data.columns + 1; // weights should match size of feature vector, plus an
                                       // extra bias term

            // create random array of weights
        let mut weights: Vec<f64> = Vec::new();
        let mut range = rand::rng();

        for _i in 0..length {
            let weight = range.random();
            weights.push( weight );
        }

        Model {
            data,
            weights
        }
    }



    pub fn verbose_new(data: Data) -> Self {

        println!("");
        println!("========== BUILDING MODEL ==========");

        let model = Model::new(data);

        println!("");
        println!("Successfully initialized model\n");

        model
    }



        // print model weights
    pub fn print(&self) {

        println!("");
        println!("========== DISPLAYING MODEL WEIGHTS ==========\n");

        for weight in &self.weights {
            println!("{}", weight);
        }
        println!("\n");
    }



    // MODEL IMPLEMENTATION
    // 
    // Update Rule: w <- w + feature * target
    //              b <- b + target
    //
    // This is the simplest case of the perceptron learning rule, where data is 2-D and linearly
    // seperable. This is essentially a form of stochastic gradient descent where the learning rate
    // is 1. To fit the model, we iterate over training examples, updating the weights only when we
    // find a misclassified example, and iterate until all examples are correctly classified


        // stochastic gradient descent
    fn sgd(&mut self, current_index: usize) {
        let target = self.data.targets[current_index];

            // iterate over weights
            // add target * feature to weight
            //
            // Features and weights are aligned by index
            // Bias is always weights[data.columns], one position beyond the final weight
        for i in 0..(self.data.columns) {
            self.weights[i] += target as f64 * self.data.elements[current_index][i];
        }

            // add target to bias
        self.weights[self.data.columns] += target as f64;
    }



        // predict from current feature vector 
    fn predict(&self, data: &Data, current_index: usize) -> isize {
        let mut hypothesis: f64 = 0.0;

        for i in 0..data.columns {
            hypothesis += self.weights[i] * data.elements[current_index][i]; 
            hypothesis += self.weights[data.columns];
        }

        if hypothesis < 0.0 { return -1; }
        1
    }



        // fit model to data
    pub fn fit(&mut self) {

        let mut misclassified = true;
        while misclassified {
            misclassified = false;
            for i in 0..self.data.rows {
                let hypothesis = self.predict(&self.data, i);
                let target = self.data.targets[i];
                if target as isize == hypothesis { continue } ;
                self.sgd(i);
                misclassified = true;
            }
        }
    }

        // fit model to data with messages
    pub fn verbose_fit(&mut self) {

        println!("");
        println!("========== TRAINING MODEL ==========");

        self.fit();

        println!("");
        println!("training complete");
    }




    // MODEL VALIDATION



        // evaluate model on testing data from file
    pub fn evaluate(&self, filename: &str) -> f64 {

        let training_data = Data::new(filename);

        let mut correct = 0;
        for i in 0..training_data.rows { 
            if self.predict(&training_data, i) == training_data.targets[i] as isize { correct += 1 }
        }
        let percent_correct = correct as f64 / training_data.rows as f64 * 100.0;

            // return accuracy
        percent_correct

    }

    pub fn verbose_evaluate(&self, filename: &str) {

        println!("");
        println!("========== EVALUATING MODEL ==========");

        let percent_correct = self.evaluate(filename);

        println!("");
        println!("Model Evaluation: {}% accuracy", percent_correct);


    }
    


        // makes prediction from user input (assumes model is 2-D)
    pub fn predict_from_xy(&self) {

        println!("");
        println!("Give me an X...");
        let x = Self::get_valid_float();

        println!("");
        println!("Give me an Y...");
        let y = Self::get_valid_float();
        
        let mut hypothesis: f64 = 0.0;

        hypothesis += self.weights[2] * x;
        hypothesis += self.weights[1] * y;
        hypothesis += self.weights[0];

        if hypothesis < 0.0 { 
            println!("");
            println!("prediction: -1");
            return;
        }
        println!("");
        println!("prediction: 1");
    }

        // gets a feature (number between -1 and 1) from the user
    fn get_valid_float() -> f64 {

        println!("input a number between -1 and 1:");

        let mut input_string = String::new();
        loop {
            io::stdin()
                .read_line(&mut input_string)
                .expect("Failed to read input");

            let input_float = input_string
                .trim()
                .parse::<f64>()
                .expect("Failed to parse input to float");

            if (input_float <= 1.0) & (input_float >= -1.0) { return input_float; }
            println!("invalid input: must be between 1 and -1");
        }
    }

}

