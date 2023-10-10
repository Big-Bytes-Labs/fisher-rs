pub mod num;
pub mod str;



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
}

pub trait StrDataSeries {
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

pub trait NumDataSeries  {
    fn mean(&self) -> f64;
    fn median(&self) -> f64;
    fn mode(&self) -> f64;
    fn min(&self) -> f64;
    fn max(&self) -> f64;
    fn sum(&self) -> f64;
    fn product(&self) -> f64;
    fn std_deviation(&self) -> f64;
}
