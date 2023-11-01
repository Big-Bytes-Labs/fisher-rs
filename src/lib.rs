/// # fisher-rs

/// [![Build Status](https://img.shields.io/github/workflow/status/Vilayat-Ali/fisher-rs/Rust%20CI?style=flat-square)](https://github.com/Vilayat-Ali/fisher-rs/actions/workflows/rust.yml)
/// [![Crates.io](https://img.shields.io/crates/v/fisher-rs.svg?style=flat-square)](https://crates.io/crates/fisher-rs)
/// [![License](https://img.shields.io/crates/l/fisher-rs.svg?style=flat-square)](LICENSE)
///
/// ![fisher-rs](./assets/logo.png)
///
/// **fisher-rs** is a Rust library that brings powerful data manipulation and analysis capabilities to Rust developers, inspired by the popular pandas library in Python. It aims to provide an intuitive and efficient way to work with structured data, making it easier to perform data wrangling, transformation, and analysis tasks in Rust projects.
///
/// **fisher-rs** is named in honor of Ronald A. Fisher, a pioneering statistician whose groundbreaking contributions have significantly shaped the field of statistics and data analysis. Fisher's innovative methodologies and concepts, including analysis of variance, maximum likelihood estimation, and the Fisher transformation, have played a vital role in modern statistical practices. Similarly, our library seeks to provide Rust developers with a set of tools inspired by Fisher's ideas, enabling them to efficiently manipulate and analyze data within the Rust programming ecosystem. Just as Fisher's work laid the foundation for statistical analysis, we hope fisher-rs will become an indispensable tool for data enthusiasts, researchers, and developers working with Rust.
///
/// ## Features
///
/// - DataFrame: A two-dimensional, size-mutable, and heterogeneous data structure.
/// - Series: A one-dimensional labeled array, capable of holding any data type.
/// - Data Alignment: Automatic data alignment during operations for hassle-free analysis.
///- Powerful Data Manipulation: Perform filtering, grouping, aggregation, and more with ease.
///- Input/Output Support: Read and write data from/to various formats, such as CSV and JSON.
///
/// ## Getting Started
///
/// To use **fisher-rs**, you'll need Rust installed on your system. Add this library to your project by including it as a dependency in your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// fisher = "0.1.0"
/// ```
///
/// Then, you can start using the library in your code:
///
/// For more examples and detailed usage, check out the documentation.
/// 

pub mod series;
pub mod errors;
pub mod results;

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