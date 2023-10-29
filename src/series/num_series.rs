pub trait NumSeries {
    fn sum(&self) -> f64;
    fn product(&self) -> f64;
    fn mean(&self) -> f64;
    fn median(&self) -> f64;
    fn mode(&self) -> f64;
    fn std_dev(&self) -> f64;
    fn min(&self) -> f64;

    fn add(&self) -> f64;
    fn sub(&self) -> f64;
    fn mul(&self) -> f64;
    fn div(&self) -> f64;

    fn corr(&self) -> f64; // calculates correlation matrix
    fn cov(&self) -> f64; // calculates covariance matrix
    fn max(&self) -> f64;
    
    fn drop_na(&self); // clears null/empty values
    fn fill_na(&self); // fill empty values
}