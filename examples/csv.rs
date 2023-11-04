use fisher::read_csv;

fn main() {
    let c = read_csv!("./data.csv");
    println!("{:#?}", c.get_col(&"name".to_string()).unwrap());

}