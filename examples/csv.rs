use fisher::series::{num_series::NumSeries, str_series::StrSeries, Series};

fn main() {
    let s: Series = StrSeries::new(vec!["ali".to_owned(), "mohd".to_owned()]);
    let mut n: Series = NumSeries::new((0..10).map(|x| x as f64).collect::<Vec<f64>>());
    n.add(2.0);
    println!("{:#?}", n);

}