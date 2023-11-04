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

use self::{str_series::StrSeries, num_series::NumSeries};

#[derive(Debug)]
pub enum SeriesType {
    Str(StrSeries),
    Num(NumSeries)
}


pub trait Series {
    fn len(&self) -> usize;
}
