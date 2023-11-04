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

pub use std::{collections::HashMap, fs::File};

use series::{SeriesType, Series, num_series::{self, NumSeries}, str_series::{self, StrSeries}};

#[macro_export]
macro_rules! read_csv {
    ($path:expr) => {
        {
            use std::{fs::File, io::Read, collections::HashMap};
            use fisher::{Dataframe, series::{is_num, SeriesType, num_series::NumSeries, str_series::StrSeries}};

            let mut csv_file: File = match File::open($path) {
                Ok(file) => file,
                Err(e) => panic!("{}", e)
            };
            let mut contents: String = String::with_capacity(1_000_000);
            match csv_file.read_to_string(&mut contents) {
                Ok(_) => {
                    let mut df_map: HashMap<String, SeriesType> = HashMap::new();

                    let rows: Vec<Vec<String>> = contents
                        // split dataset buffer string into rows
                        .split('\n')
                        .map(|row_str| row_str.to_owned())
                        .collect::<Vec<String>>()
                        .into_iter()
                        .map(|row| {
                            // splitting rows into cells
                            row.split(',')
                                .map(|cell_str| cell_str.to_owned())
                                .collect::<Vec<String>>()
                                .iter()
                                .map(|cell| {
                                    cell.trim().to_owned()
                                })
                                .collect::<Vec<String>>()
                        }).collect::<Vec<Vec<String>>>();

                    let size: (usize, usize) = (rows.len(), rows[0].len());

                    for cell_idx in 0..rows[0].len() {
                        match is_num!(&rows[1][cell_idx]) {
                            true => {
                                let vec: Vec<f64> = (1..rows.len()).map(|row_idx| {
                                    rows[row_idx][cell_idx].parse::<f64>().unwrap()
                                }).collect::<Vec<f64>>();
                                df_map.insert(rows[0][cell_idx].to_string(), SeriesType::Num(NumSeries(
                                    vec
                                )));
                            },
                            false => {
                                let vec: Vec<String> = (1..rows.len()).map(|row_idx| {
                                    rows[row_idx][cell_idx].to_owned()
                                }).collect::<Vec<String>>();
                                df_map.insert(rows[0][cell_idx].to_string(), SeriesType::Str(StrSeries(
                                    vec
                                )));
                            }
                        }
                    }
                        
                    Dataframe {
                        frame: df_map,
                        size
                    }
                },
                Err(e) => panic!("{}", e)
            }
        }
    };
}

#[derive(Debug)]

pub struct Dataframe {
    pub frame: HashMap<String, SeriesType>,
    pub size: (usize, usize),
}

impl Dataframe {
    pub fn df(&self) -> &HashMap<String, SeriesType> {
            &self.frame
    }

    pub fn df_mut(&mut self) -> &mut HashMap<String, SeriesType> {
        &mut self.frame
    }

    pub fn get_col<'a>(&self, col_header: &'a str) -> Option<&SeriesType>
        {
           match self.frame.get(col_header) {
                Some(series) => {
                    Some(series)
                    // match series {
                    //     SeriesType::Num(series) => Some(series),
                    //     SeriesType::Str(series) => Some(series)
                    // }
                },
                None => None,
            }
        }
}