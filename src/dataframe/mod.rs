use std::{collections::HashMap, fs::File, io::{self, BufRead}};

/// A data frame in **fisher-rs**, is a Two-Dimensional data structure, portenstitially heterogeneous tabular data structure with labeled 
/// axes rows, and columns.
/// 
/// ## Features of a data frame
/// 
/// - Potentially columns are of different types.
/// - Pandas DataFrame size is mutable.
/// - DataFrame labeled axes (rows and columns).
/// - can perform arithmetic operations on rows and columns on DataFrame.
/// 
#[derive(Debug)]
pub struct DataFrame {
    pub frame: HashMap<String, String>,
    pub size: (usize, usize),
}

impl DataFrame {

    // creating a data frame from a csv file
    pub fn from_csv(file_path: &str, _delimiter: Option<&'static str>) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut frame: HashMap<String, Vec<String>> = HashMap::new();

        for line in reader.lines() {
            let record: Vec<String> = line?.split(',').map(|s| s.to_string()).collect::<Vec<String>>();
            println!("{:?}", record);
        }

        Ok(Self {
            frame: HashMap::new(),
            size: (0, 0)
        })
    }

    pub fn from_json() -> Self {
        Self {
            frame: HashMap::new(),
            size: (0, 0)
        }
    }
}