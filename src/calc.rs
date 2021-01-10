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
        let mut total: f64 = arr[0] as f64;
        let mut counter = 0;

        while counter != arr.len() - 1 {
            total -= arr[counter + 1] as f64;
            counter += 1;
        }

        total
    }

    pub fn mul(arr: Vec<i64>) -> f64 {
        let mut total: f64 = arr[0] as f64;

        let mut counter = 0;

        while counter != arr.len() - 1 {
            total *= arr[counter + 1] as f64;
            counter += 1;
        }

        total
    }

    pub fn div(arr: Vec<i64>) -> f64 {
        let mut total: f64 = arr[0] as f64;

        let mut counter = 0;

        while counter != arr.len() - 1 {
            total /= arr[counter + 1] as f64;
            counter += 1;
        }

        total
    }
}

#[test]
fn test_all_operations() {
    // addition
    assert_eq!(Calc::add([2, 4, 6].to_vec()), 12.0);
    assert_eq!(Calc::add([-6, 5, 10].to_vec()), 9.0);

    // subtraction
    assert_eq!(Calc::sub([10, 4, 6].to_vec()), 0.0);
    assert_eq!(Calc::sub([100, 10, 19].to_vec()), 71.0);

    // multiplication
    assert_eq!(Calc::mul([10, 10, 2].to_vec()), 200.0);
    assert_eq!(Calc::mul([-3, 2].to_vec()), -6.0);

    // division
    assert_eq!(Calc::div([54, 2, 3].to_vec()), 9.0);
    assert_eq!(Calc::div([4, 2, 5].to_vec()), 0.4);
}
