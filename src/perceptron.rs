use std::fs;
use rand::Rng;

    // data struct
pub struct Data {
    elements: Vec<Vec<f64>>,
    targets: Vec<i32>,
    rows: usize,
    columns: usize,
}

impl Data {

        // populate data from file
        
    pub fn new(filename: &str) -> Self {

        println!("");
        println!("========== LOADING DATA ==========");

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

        println!("");
        println!("Successfully loaded data from file: {}! \n", filename);

            // return populated data struct
        Data {
            elements,
            targets,
            rows,
            columns
        }
    }

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

    // populate model
    pub fn new(data: Data) -> Self {

        println!("");
        println!("========== BUILDING MODEL ==========");

        let length = data.columns + 1;

            // create random array of weights
        let mut weights: Vec<f64> = Vec::new();
        let mut range = rand::thread_rng();

        for _i in 0..length {
            let weight = range.gen();
            weights.push( weight );
        }

        println!("");
        println!("Successfully initialized model\n");

        Model {
            data,
            weights
        }
    }

    pub fn print(&self) {

        println!("");
        println!("========== DISPLAYING MODEL WEIGHTS ==========\n");

        for weight in &self.weights {
            println!("{}", weight);
        }
        println!("\n");
    }


}

