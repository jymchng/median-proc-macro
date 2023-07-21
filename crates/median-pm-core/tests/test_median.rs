use median_pm_core::median;
use std::any::TypeId;

fn get_type_name<T>(value: &T) -> &str {
    std::any::type_name::<T>()
}

#[test]
fn test_median_odd_elements() {
    let numbers = vec![1, 2, 3, 4, 5];
    assert_eq!(median!(1, 2, 3, 4, 5), 3);
    assert_eq!(get_type_name(&median!(1, 2, 3, 4, 5)), "i32");
}

#[test]
fn test_median_even_elements() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(median!(1, 2, 3, 4, 5, 6), 3.5);
    assert_eq!(get_type_name(&median!(1, 2, 3, 4, 5, 6)), "f64");
}

#[test]
fn test_median_zero_elements() {
    let numbers: Vec<u64> = Vec::new();
    assert_eq!(median!(), 0.0);
    assert_eq!(get_type_name(&median!()), "f64");
}

#[test]
fn test_median_single_element() {
    assert_eq!(median!(42), 42);
    assert_eq!(get_type_name(&median!(42)), "i32");
}

#[test]
fn test_median_two_elements() {
    let numbers = vec![10, 20];
    assert_eq!(median!(10, 20), 15.0);
    assert_eq!(get_type_name(&median!(10, 20)), "f64");
}

#[test]
fn test_median_three_elements() {
    let numbers = vec![10, 20, 30];
    assert_eq!(median!(10, 20, 30), 20);
    assert_eq!(get_type_name(&median!(10, 20, 30)), "i32");
}