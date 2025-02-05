
use std::fs;

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

        println!("Successfully loaded data from file: {}! \n", filename);

        Data {
            elements,
            targets,
            rows,
            columns
        }
    }

    pub fn print(&self) {
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

