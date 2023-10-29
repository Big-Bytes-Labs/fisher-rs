pub mod num_series;
pub mod str_series;

#[macro_export]
macro_rules! is_num {
    ($val: expr) => {
        {
            fn convert_to_string<T: Into<String>>(value: T) -> String {
                value.into()
            }

            let val: String = convert_to_string($val);
            match val.trim().to_owned().parse::<f64>() {
                Ok(_) => true,
                Err(_) => false
            }
        }
    };
}

pub use is_num;

#[derive(Debug)]
pub enum Series {
    Str(Vec<String>),
    Num(Vec<f64>),
}

impl Series {
    pub fn len(&self) -> usize {
        match &self {
            Self::Str(ref v) => v.len(),
            Self::Num(ref v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self {
            Self::Str(ref v) => v.is_empty(),
            Self::Num(ref v) => v.is_empty(),
        }
    }
}
