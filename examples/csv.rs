use fisher::series::Series;
use fisher::series::{str_series::StrSeries, num_series::NumSeries};

fn main() {
    let s: Series = Series::Str(vec!["ali".to_owned(), "mohd".to_owned()]);
    let mut n: Series = Series::Num((0..10).into_iter().map(|x| x as f64).collect::<Vec<f64>>());
    // n.add(2.0);
    println!("{:#?}", n);
}