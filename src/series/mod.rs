pub enum SeriesType {
    Str(Box<Vec<String>>),
    Flt(Box<Vec<f64>>),
    Int(Box<Vec<usize>>),
}

pub struct Series (SeriesType);

impl Series {
    pub fn new() -> Self {
        Self (
            SeriesType::Int(Box::new(Vec::new()))
        )
    }
}

pub trait StrSeries {}

pub trait FltSeries {}

pub trait IntSeries {}
