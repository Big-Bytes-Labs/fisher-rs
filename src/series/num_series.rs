use super::Series;

pub trait NumSeries {
    fn sum(&self) -> Option<f64>;
    fn product(&self) -> Option<f64>;
    fn mean(&self) -> Option<f64>;
    fn median(&self) -> Option<f64>;
    fn mode(&self) -> Option<f64>;
    fn std_dev(&self) -> Option<f64>;
    fn min(&self) -> Option<f64>;

    fn add(&self) -> Option<f64>;
    fn sub(&self) -> Option<f64>;
    fn mul(&self) -> Option<f64>;
    fn div(&self) -> Option<f64>;

    fn corr(&self) -> Option<f64>; // calculates correlation matrix
    fn cov(&self) -> Option<f64>; // calculates covariance matrix
    fn max(&self) -> Option<f64>;
    
    fn drop_na(&self); // clears null/empty values
    fn fill_na(&self); // fill empty values
}

impl NumSeries for Series {
    fn sum(&self) -> Option<f64> {
        if let Series::Num(ref vec) = &self {
            Some(vec.iter().sum::<f64>())
        } else {
            None
        }
    }

    fn product(&self) -> Option<f64> {
        if let Series::Num(ref vec) = &self {
            Some(vec.iter().product::<f64>())
        } else {
            None
        }
    }

    fn mean(&self) -> Option<f64> {
        if let Series::Num(ref vec) = &self {
            Some(vec.iter().sum::<f64>() / vec.len() as f64)
        } else {
            None
        }
    }

    fn median(&self) -> Option<f64> {
        todo!()
    }

    fn mode(&self) -> Option<f64> {
        todo!()
    }

    fn std_dev(&self) -> Option<f64> {
        todo!()
    }

    fn min(&self) -> Option<f64> {
        if let Series::Num(ref vec) = &self {
            if vec.len() == 0 {
                return None;
            }

            let mut vec = vec.clone();
            vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(vec[0])
        } else {
            None
        }
    }

    fn max(&self) -> Option<f64> {
        if let Series::Num(ref vec) = &self {
            if vec.len() == 0 {
                return None;
            }

            let mut vec = vec.clone();
            vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(vec[vec.len() - 1])
        } else {
            None
        }
    }

    fn add(&self) -> Option<f64> {
        todo!()
    }

    fn sub(&self) -> Option<f64> {
        todo!()
    }

    fn mul(&self) -> Option<f64> {
        todo!()
    }

    fn div(&self) -> Option<f64> {
        todo!()
    }

    fn corr(&self) -> Option<f64> {
        todo!()
    }

    fn cov(&self) -> Option<f64> {
        todo!()
    }

    fn drop_na(&self) {
        todo!()
    }

    fn fill_na(&self) {
        todo!()
    }
}