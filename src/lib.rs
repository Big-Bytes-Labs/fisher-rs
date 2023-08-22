use std::{collections::HashMap, default::Default, fmt};

mod math;

#[derive(Debug)]
pub struct DF {
    pub frame: HashMap<String, Vec<String>>,
    pub shape: (usize, usize),
}

impl Default for DF {
    fn default() -> Self {
        Self {
            frame: HashMap::new(),
            shape: (0, 0),
        }
    }
}

impl fmt::Display for DF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hello")
    }
}

impl DF {
    pub fn read_csv(path_to_csv: &str) -> Result<String, String> {
        Ok(String::new())
    }
}
