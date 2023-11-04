use crate::is_num;
use super::Series;

#[derive(Debug)]
pub struct StrSeries(pub Vec<String>);

impl StrSeries {
    pub fn new<T>(data: T) -> Self 
    where 
        T: Into<Vec<String>>
    {
        StrSeries(data.into())
    }

    pub fn append(&mut self, element: impl Into<String>) {
        self.0.push(element.into());
    }

    pub fn pop(&mut self) -> Option<String> {
        self.0.pop()
    }

    pub fn remove(&mut self, index: usize) -> String {
        self.0.remove(index)
    }

    pub fn to_uppercase(&mut self) {
        self.0.iter_mut().for_each(|str_data| *str_data = str_data.to_uppercase());
    }

    pub fn to_lowercase(&mut self) {
        self.0.iter_mut().for_each(|str_data| *str_data = str_data.to_lowercase());
    }

    pub fn average_char_count(&self) -> f64 {
            let sum_of_chars: usize = self.0.iter().map(|str_data| {
                str_data.len()
            }).sum();

            sum_of_chars as f64 / self.0.len() as f64
    }

    pub fn is_alpha(&self) -> bool {
        for el in self.0.iter() {
            if is_num!(el) {
                return false;
            }
        }
        true
    }

    pub fn is_num(&self) -> bool {
        for el in self.0.iter() {
            if !is_num!(el) {
                return false;
            }
        }
        true
    }    

    pub fn cat(&self, separator: Option<&str>) -> String {
        self.0.iter().map(|data_str| data_str.to_owned()).collect::<Vec<String>>().join(separator.unwrap_or(""))
    }

    pub fn strip(&mut self) {
        self.0.iter_mut().for_each(|data_str| *data_str = data_str.trim().to_string());
    }
}

impl Series for StrSeries {
    fn len(&self) -> usize {
        self.0.len()
    }
}