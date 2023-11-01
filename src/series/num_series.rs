use std::collections::HashMap;

use super::Series;

pub trait NumSeries {
    fn sum(&self) -> Option<f64>;
    fn product(&self) -> Option<f64>;

    fn mean(&self) -> Option<f64>;
    fn median(&self) -> Option<f64>;
    fn mode(&self) -> Option<f64>;
    fn std_dev(&self) -> Option<f64>;

    fn max(&self) -> Option<f64>;
    fn min(&self) -> Option<f64>;

    fn add(&mut self, num: f64);
    fn sub(&mut self, num: f64);
    fn mul(&mut self, num: f64);
    fn div(&mut self, num: f64);

    fn replace_with(&mut self, val: Option<f64>); 
    fn filter<T>(&self, predicate: T) -> Option<Vec<f64>> where T: FnMut(&f64) -> bool;
}

impl NumSeries for Series {
    fn sum(&self) -> Option<f64> {
        if let Series::Num(ref vec) = &self {
            Some(vec.iter().sum::<f64>())
        } else {
            None
        }
    }

    fn product(&self) -> Option<f64> {
        if let Series::Num(ref vec) = &self {
            Some(vec.iter().product::<f64>())
        } else {
            None
        }
    }

    fn mean(&self) -> Option<f64> {
        if let Series::Num(ref vec) = &self {
            Some(vec.iter().sum::<f64>() / vec.len() as f64)
        } else {
            None
        }
    }

    fn median(&self) -> Option<f64> {
        if let Series::Num(ref vec) = &self {
            if vec.len() == 0 {
                return None;
            }
            let mut sorted_vec: Vec<f64> = vec.clone();
            sorted_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
            match sorted_vec.len() % 2 {
                0 => Some(sorted_vec[sorted_vec.len() / 2]),
                _ => Some(sorted_vec[(sorted_vec.len() + 1) / 2])
            }
        } else {
            None
        }
    }

    fn mode(&self) -> Option<f64> {
        if let Series::Num(ref vec) = &self {
            let mut frequency_map: HashMap<String, usize> = HashMap::new();
            for el in vec.iter() {
                let count = frequency_map.entry(el.to_string()).or_insert(0);
                *count += 1;
            }
            match frequency_map.into_iter().max_by_key(|el| el.1) {
                Some((el, _)) => Some(el.parse::<f64>().unwrap()),
                None => None
            }
        } else {
            None
        }
    }

    fn std_dev(&self) -> Option<f64> {
        if let Series::Num(ref vec) = self {
            match self.mean() {
                Some(mean) => {
                    let squared_diff_sum: f64 = vec.clone().into_iter().map(|el| ((el - mean) as f64).powi(2)).sum();
                    let variance = squared_diff_sum / (vec.len() as f64);
                    Some(variance.sqrt())
                },
                None => None
            }
        } else {
            None
        }
    }

    fn min(&self) -> Option<f64> {
        if let Series::Num(ref vec) = &self {
            if vec.len() == 0 {
                return None;
            }

            let mut vec = vec.clone();
            vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(vec[0])
        } else {
            None
        }
    }

    fn max(&self) -> Option<f64> {
        if let Series::Num(ref vec) = &self {
            if vec.len() == 0 {
                return None;
            }

            let mut vec = vec.clone();
            vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(vec[vec.len() - 1])
        } else {
            None
        }
    }

    fn add(&mut self, num: f64) {
        if let Series::Num(ref mut vec) = self {
            vec.iter_mut().for_each(|el| *el += num)
        } 
    }

    fn sub(&mut self, num: f64) {
        if let Series::Num(ref mut vec) = self {
            vec.iter_mut().for_each(|el| *el -= num)
        } 
    }

    fn mul(&mut self, num: f64) {
        if let Series::Num(ref mut vec) = self {
            vec.iter_mut().for_each(|el| *el *= num)
        } 
    }

    fn div(&mut self, num: f64) {
        if let Series::Num(ref mut vec) = self {
            vec.iter_mut().for_each(|el| *el = *el / num)
        } 
    }

    fn replace_with(&mut self, val: Option<f64>) {
        if let Series::Num(ref mut vec) = self {
            vec.iter_mut()
                .for_each(|el| {
                    *el = val.unwrap_or_default()
                })
        }
    }

    fn filter<T>(&self, predicate: T) -> Option<Vec<f64>> where T: FnMut(&f64) -> bool {
        if let Series::Num(ref vec) = self {
            Some(vec.clone().into_iter()
                .filter(predicate)
                .collect::<Vec<f64>>())
        } else {
            None
        }
    }

}