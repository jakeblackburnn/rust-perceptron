pub fn hello() {
    println!("Hello, world!");
}


    // data struct
pub struct Data {
    elements: Vec<Vec<f64>>,
    targets: Vec<i32>,
    rows: usize,
    columns: usize,
}

impl Data {

    pub fn new() -> Self {

        Data {
            elements: vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],
            ],
            targets: vec![0, 1],
            rows: 2,
            columns: 3,
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

