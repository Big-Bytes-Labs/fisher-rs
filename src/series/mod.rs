use std::error::Error;

#[derive(Debug, Clone)]
pub enum Series {
    Str(Vec<String>),
    Int32(Vec<i32>),
    Int64(Vec<i64>),
    Float32(Vec<f32>),
    Float64(Vec<f64>),
}

impl Series {
    pub fn len(&self) -> usize {
        match self {
            Self::Str(vec) => vec.len(),
            Self::Int32(vec) => vec.len(),
            Self::Int64(vec) => vec.len(),
            Self::Float32(vec) => vec.len(),
            Self::Float64(vec) => vec.len(),
        }
    }
    pub fn mean(&self) -> f64 {
        match self {
            Self::Int32(vec) => vec.iter().map(|&x| x as f64).sum::<f64>() / vec.len() as f64,
            Self::Int64(vec) => vec.iter().map(|&x| x as f64).sum::<f64>() / vec.len() as f64,
            Self::Float32(vec) => vec.iter().map(|&x| x as f64).sum::<f64>() / vec.len() as f64,
            Self::Float64(vec) => vec.iter().map(|&x| x as f64).sum::<f64>() / vec.len() as f64,
            _ => panic!("Operation not supported!"),
        }
    }
    pub fn median(&self) -> f64 {
        match self {
            Self::Int32(vec) => {
                let mut vec = vec.clone(); 
                vec.sort();
                vec[vec.len() / 2] as f64
            },
            Self::Int64(vec) => {
                let mut vec = vec.clone();
                vec.sort();
                vec[vec.len() / 2] as f64
            },
            Self::Float32(vec) => {
                let mut vec = vec.clone();
                vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
                vec[vec.len() / 2] as f64
            },
            Self::Float64(vec) => {
                let mut vec = vec.clone();
                vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
                vec[vec.len() / 2] as f64
            },
            _ => panic!("Operation not supported!"),
        } 
    }
}

pub trait StrDataSeries<T> where T: Clone {
    fn concat(&self) -> String;
    fn contains(&self, pattern: &'static str) -> bool;
    fn join<'a>(&self, separator: &'a str) -> String;
    fn upper(&self) -> String;
    fn lower(&self) -> String;
    fn is_alpha_num(&self) -> bool;
    fn is_integer_num(&self) -> bool;
    fn is_decimal_num(&self) -> bool;
    fn is_alpha(&self) -> bool;
}

pub trait NumDataSeries<T> where T: PartialEq + PartialOrd {
    fn mean(&self) -> f64;
    fn median(&self) -> T;
    fn mode(&self) -> T;
    fn min(&self) -> T;
    fn max(&self) -> T;
    fn sum(&self) -> T;
    fn product(&self) -> Result<T, Box<dyn Error>>;
    fn std_deviation(&self) -> f64;
}

