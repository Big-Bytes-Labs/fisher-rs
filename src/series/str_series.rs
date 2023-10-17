use crate::series::Series;

pub trait StrSeries {
    fn new(data: Vec<String>) -> Self;
    fn append(&mut self, element: impl Into<String>);
    fn pop(&mut self) -> Result<String, String>;
    fn remove(&mut self, index: usize) -> Result<String, String>;
}

impl StrSeries for Series {
    fn new(data: Vec<String>) -> Self {
        Self (
            super::SeriesType::Str(data)
        )
    }

    fn append(&mut self, element: impl Into<String>) {
        if let super::SeriesType::Str(ref mut data) = &mut self.0 {
            data.push(element.into());
        }
    }

    fn pop(&mut self) -> Result<String, String> {
        todo!()
    }

    fn remove(&mut self, _index: usize) -> Result<String, String> {
        todo!()
    }

}