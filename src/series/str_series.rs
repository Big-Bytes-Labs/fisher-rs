use crate::series::Series;
use crate::is_num;

pub trait StrSeries {
    fn new(data: Vec<String>) -> Self;
    fn append(&mut self, element: impl Into<String>);
    fn pop(&mut self) -> Option<String>;
    fn remove(&mut self, index: usize) -> Option<String>;
    fn to_uppercase(&mut self);
    fn to_lowercase(&mut self);
    fn average_char_count(&self) -> Option<f64>;
    fn is_alpha(&self) -> Option<bool>;
    fn is_num(&self) -> Option<bool>;
    fn cat(&self, separator: Option<&str>) -> Option<String>;
    fn strip(&mut self);
}

impl StrSeries for Series {
    fn new(data: Vec<String>) -> Self {
        Self::Str(data)
    }

    fn append(&mut self, element: impl Into<String>) {
        if let Series::Str(ref mut vec) = self {
            vec.push(element.into());
        }
    }

    fn pop(&mut self) -> Option<String> {
        if let Series::Str(ref mut vec) = self {
            vec.pop()
        } else {
            None
        }
    }

    fn remove(&mut self, index: usize) -> Option<String> {
        if let Series::Str(ref mut vec) = self {
            Some(vec.remove(index))
        } else {
            None
        }
    }

    fn to_uppercase(&mut self) {
        if let Series::Str(ref mut vec) = self {
            vec.iter_mut().for_each(|str_data| *str_data = str_data.to_uppercase());
        }
    }

    fn to_lowercase(&mut self) {
        if let Series::Str(ref mut vec) = self {
            vec.iter_mut().for_each(|str_data| *str_data = str_data.to_lowercase());
        }
    }

    fn average_char_count(&self) -> Option<f64> {
        if let Series::Str(ref vec) = self {
            let sum_of_chars: usize = vec.iter().map(|str_data| {
                str_data.len()
            }).sum();

            Some(sum_of_chars as f64 / vec.len() as f64)
        } else {
            None
        }
    }

    fn is_alpha(&self) -> Option<bool> {
        if let Series::Str(ref vec) = self {
            for el in vec.iter() {
                if is_num!(el) {
                    return Some(false);
                }
            }
            Some(true)
        } else {
            None
        }
    }

    fn is_num(&self) -> Option<bool> {
        if let Series::Str(ref vec) = self {
            for el in vec.iter() {
                if !is_num!(el) {
                    return Some(false);
                }
            }
            Some(true)
        } else {
            None
        }
    }

    fn cat(&self, separator: Option<&str>) -> Option<String> {
        if let Series::Str(ref vec) = self {
            Some(vec.iter().map(|data_str| data_str.to_owned()).collect::<Vec<String>>().join(separator.unwrap_or("")))
        } else {
            None
        }
    }

    fn strip(&mut self) {
        if let Series::Str(ref mut vec) = self {
            vec.iter_mut().for_each(|data_str| *data_str = data_str.trim().to_string());
        }
    }

}