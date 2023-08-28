//! # Fisher-rs
//! 
//! **fisher-rs** is a Rust library that brings powerful data manipulation and analysis capabilities to Rust developers,
//! inspired by the popular pandas library in Python. It aims to provide an intuitive and efficient way to work with 
//! structured data, making it easier to perform data wrangling, transformation, and analysis tasks in Rust projects.
//! 
//! 
//! ## Inspiration
//! 
//! **fisher-rs** is named in honor of Ronald A. Fisher, a pioneering statistician whose groundbreaking contributions 
//! have significantly shaped the field of statistics and data analysis. Fisher's innovative methodologies and concepts,
//!  including analysis of variance, maximum likelihood estimation, and the Fisher transformation, have played a vital 
//! role in modern statistical practices. Similarly, our library seeks to provide Rust developers with a set of tools 
//! inspired by Fisher's ideas, enabling them to efficiently manipulate and analyze data within the Rust programming 
//! ecosystem. Just as Fisher's work laid the foundation for statistical analysis, we hope fisher-rs will become an 
//! indispensable tool for data enthusiasts, researchers, and developers working with Rust.
//! 
//! ## Features
//! 
//! - **DataFrame**: A two-dimensional, size-mutable, and heterogeneous data structure.
//! - **Series**: A one-dimensional labeled array, capable of holding any data type.
//! - **Data Alignment**: Automatic data alignment during operations for hassle-free analysis.
//! - **Powerful Data Manipulation**: Perform filtering, grouping, aggregation, and more with ease.
//! - **Input/Output Support**: Read and write data from/to various formats, such as CSV and JSON.
//! 
//! ## Basic Usage
//! 
//! First include **fisher** library into your rust project.
//! 
//! Add fisher to your Cargo.toml file
//! 
//! ```
//! [dependencies]
//! fisher = "0.1.0"
//! ```
//! 
//! Or, run `cargo add fisher` in terminal at root of your project
//! 
//! ```
//! use fisher::prelude::*;
//! 
//! fn main() {
//!   // Create a DataFrame
//!   let mut df = DataFrame::new();
//!   df.add_column("name", Series::new(vec!["Alice", "Bob", "Charlie"]));
//!   df.add_column("age", Series::new(vec![25, 30, 22]));

//!   // Perform data manipulation
//!   let filtered = df.filter(df.column("age").gt(25));

//!   // Display the result
//!   println!("{}", filtered);
//! }
//! ```

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub mod dataframe;

pub mod prelude;