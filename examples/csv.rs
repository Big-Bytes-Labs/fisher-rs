pub use fisher::prelude::*;
use fisher::series::Series;
use fisher::series::str_series::StrSeries;

fn main() {
    let mut s = Series::Str(vec!["ali".to_owned(), "mohd".to_owned()]);
    s.to_uppercase();

    println!("{:#?}", s);
}