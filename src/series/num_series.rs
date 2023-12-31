pub trait NumSeries {
    fn mean(&self) -> isize;
    fn median(&self) -> isize;
    fn mode(&self) -> isize;
}

impl<T> NumSeries for Vec<T> 
    where T: Sized {
        fn mean(&self) -> isize {
            todo!()
        }

        fn median(&self) -> isize {
            todo!()
        }

        fn mode(&self) -> isize {
            todo!()
        }
    }