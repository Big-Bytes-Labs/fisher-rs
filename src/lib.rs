use std::{io, fs, collections::HashMap};
use std::fmt::Display;

#[macro_export]
macro_rules! read_csv {
    ($path: expr) => {
        {
            use std::{
                fs::File,
                io::Read,
                collections::HashMap
            };

            match File::open($path) {
                Ok(mut csv_file) => {
                    let mut contents = String::new();
                    match csv_file.read_to_string(&mut contents) {
                        Ok(_) => {
                            let mut data_frame: HashMap<String, Vec<String>> = HashMap::new();
                    
                            let mut rows = contents.split('\n')
                                .into_iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .into_iter()
                                .map(|row| {
                                    row.split(',')
                                    .map(|cell| cell.trim().to_owned())
                                    .collect::<Vec<String>>()
                                }).collect::<Vec<Vec<String>>>();

                            match rows.len() {
                                0 => {
                                    println!("None");
                                },
                                _ => {
                                    
                                }
                            }

                            println!("{:#?}", rows);
                        },
                        Err(e) => {}
                    }
                },
                Err(e) => {}
            }
        }
    };
}

#[derive(Debug)]
pub struct Fisher {
    pub df: HashMap<String, Vec<String>>,
    pub shape: (usize, usize)
}

impl Display for Fisher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Fisher {
    pub fn head() {}

    pub fn describe() {}

}