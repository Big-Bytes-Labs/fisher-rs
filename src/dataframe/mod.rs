use std::{collections::HashMap, fs::File, io::{Read}};
 
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
pub struct DataFrame {
    pub frame: HashMap<String, String>,
    pub size: (usize, usize),
}

impl DataFrame {

    // creating a data frame from a csv file
    pub fn from_csv(file_path: &str, delimiter: Option<&'static str>) -> Result<Self, Box<dyn std::error::Error>> {

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