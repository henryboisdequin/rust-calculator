#![allow(dead_code)]

pub struct Calc;

impl Calc {
    pub fn add(arr: Vec<i64>) -> f64 {
        let mut total: f64 = 0.0;

        for num in arr {
            total += num as f64;
        }

        total
    }

    pub fn sub(arr: Vec<i64>) -> f64 {
        let mut total: f64 = 0.0;

        for num in arr {
            total -= num as f64;
        }

        total
    }

    pub fn mul(arr: Vec<i64>) -> f64 {
        let mut total: f64 = 0.0;

        for num in arr {
            total *= num as f64;
        }

        total
    }

    pub fn div(arr: Vec<i64>) -> f64 {
        let mut total: f64 = 0.0;

        for num in arr {
            total /= num as f64;
        }

        total
    }
}
