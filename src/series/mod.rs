pub mod float_series;
pub mod int_series;
pub mod str_series;

#[derive(Debug)]
pub enum SeriesType {
    Str(Vec<String>),
    Flt(Vec<f64>),
    Int(Vec<usize>),
}

#[derive(Debug)]
pub struct Series (SeriesType);

impl Series {
    pub fn len(&self) -> usize {
        match &self.0 {
            SeriesType::Str(ref v) => v.len(),
            SeriesType::Flt(ref v) => v.len(),
            SeriesType::Int(ref v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.0 {
            SeriesType::Str(ref v) => v.is_empty(),
            SeriesType::Flt(ref v) => v.is_empty(),
            SeriesType::Int(ref v) => v.is_empty(),
        }
    }
}
