use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Sub},
};

pub fn mean<T>(arr: Vec<T>) -> f64
where
    T: PartialOrd + Default + Add + Sub + Mul + Div + AddAssign + std::fmt::Display + Copy,
{
    let mut total: T = T::default();

    for num in arr.iter() {
        total += *num;
    }

    (total.to_string().parse::<f64>().unwrap()) / (arr.len() as f64)
}

pub fn median<T>(arr: Vec<T>) -> f64
where
    T: PartialOrd + Default + Add + Sub + Mul + Div + AddAssign + std::fmt::Display + Copy,
{
    let mut total: T = T::default();

    for num in arr.iter() {
        total += *num;
    }

    (total.to_string().parse::<f64>().unwrap()) / (arr.len() as f64)
}

pub fn mode<T>(arr: Vec<T>) -> f64
where
    T: PartialOrd + Default + Add + Sub + Mul + Div + AddAssign + std::fmt::Display + Copy,
{
    let mut total: T = T::default();

    for num in arr.iter() {
        total += *num;
    }

    (total.to_string().parse::<f64>().unwrap()) / (arr.len() as f64)
}

pub fn std_deviation<T>(arr: Vec<T>) -> f64
where
    T: PartialOrd + Default + Add + Sub + Mul + Div + AddAssign + std::fmt::Display + Copy,
{
    let mut total: T = T::default();

    for num in arr.iter() {
        total += *num;
    }

    (total.to_string().parse::<f64>().unwrap()) / (arr.len() as f64)
}
