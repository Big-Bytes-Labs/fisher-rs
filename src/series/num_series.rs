use std::collections::HashMap;
use super::Series;

#[derive(Debug)]
pub struct NumSeries(pub Vec<f64>);

impl NumSeries {
    pub fn new<T>(data: T) -> Self 
    where 
        T: Into<Vec<f64>> 
    {
        Self(data.into())
    }

    pub fn sum(&self) -> f64 {
        self.0.iter().sum::<f64>()
    }

    pub fn product(&self) -> f64 {
        self.0.iter().product::<f64>()
    }

    pub fn mean(&self) -> f64 {
        self.0.iter().sum::<f64>() / self.0.len() as f64
    }

    pub fn median(&self) -> Option<f64> {
        if self.0.is_empty() {
            return None;
        }
        let mut sorted_vec: Vec<f64> = self.0.clone();
        sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        match sorted_vec.len() % 2 {
            0 => Some(sorted_vec[sorted_vec.len() / 2]),
            _ => Some(sorted_vec[(sorted_vec.len() + 1) / 2])
        }
    }

    pub fn mode(&self) -> Option<f64> {
        if self.0.is_empty() {
            None
        } else {
            let mut frequency_map: HashMap<String, usize> = HashMap::new();
            for el in self.0.iter() {
                let count = frequency_map.entry(el.to_string()).or_insert(0);
                *count += 1;
            }
            frequency_map.into_iter().max_by_key(|el| el.1).map(|(el, _)| el.parse::<f64>().unwrap())
        }
    }

    pub fn std_dev(&self) -> Option<f64> {
        if self.0.is_empty() {
            None
        } else {
            let squared_diff_sum: f64 = self.0.clone().into_iter().map(|el| (el - self.mean()).powi(2)).sum();
            let variance = squared_diff_sum / (self.0.len() as f64);
            Some(variance.sqrt())
        }
    }


    pub fn min(&self) -> Option<f64> {
        if self.0.is_empty() {
            None
        } else {
            let mut vec = self.0.clone();
            vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(vec[0])
        }
    }

    pub fn max(&self) -> Option<f64> {
        if self.0.is_empty() {
            None
        } else {
            let mut vec = self.0.clone();
            vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(vec[vec.len() - 1])
        }
    }

    pub fn add(&mut self, num: f64) {
        self.0.iter_mut().for_each(|el| *el += num) 
    }

    pub fn sub(&mut self, num: f64) {
        self.0.iter_mut().for_each(|el| *el -= num)
    }

    pub fn mul(&mut self, num: f64) {
        self.0.iter_mut().for_each(|el| *el *= num)
    }

    pub fn div(&mut self, num: f64) {
        self.0.iter_mut().for_each(|el| *el /= num)
    }

    pub fn replace_with(&mut self, val: Option<f64>) {
        self.0.iter_mut()
                .for_each(|el| {
                    *el = val.unwrap_or_default()
                })
    }

    pub fn filter<T>(&self, predicate: T) -> Vec<f64> 
        where T: FnMut(&f64) -> bool 
    {
       self.0.clone().into_iter()
                .filter(predicate)
                .collect::<Vec<f64>>()
    }

}

impl Series for NumSeries {
    fn len(&self) -> usize {
        self.0.len()
    }
}