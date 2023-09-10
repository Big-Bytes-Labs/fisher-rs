use std::error::Error;

#[derive(Debug, Clone)]
pub struct Series<T>(Vec<T>);

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

