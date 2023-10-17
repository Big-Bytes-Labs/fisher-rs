pub use crate::{series, errors, results};
pub use std::collections::HashMap;

pub struct Dataframe {
    pub frame: HashMap<String, String>,
    pub size: (usize, usize),
}

pub trait DataFrame {
    fn generate_frame() -> Self;

    fn print_first(&self);

    fn print_last(&self);
}

impl DataFrame for Dataframe {
    fn generate_frame() -> Self {
        todo!()
    }

    fn print_first(&self) {
        
    }

    fn print_last(&self) {
        
    }
}