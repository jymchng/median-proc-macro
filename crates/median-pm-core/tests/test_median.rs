use median_pm_core::median;

#[test]
fn test_median_odd_elements() {
    let numbers = vec![1, 2, 3, 4, 5];
    assert_eq!(median!(1, 2, 3, 4, 5), 3);
}

#[test]
fn test_median_even_elements() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(median!(1, 2, 3, 4, 5, 6), 3.5);
}

#[test]
fn test_median_zero_elements() {
    let numbers: Vec<u64> = Vec::new();
    assert_eq!(median!(), 0);
}

#[test]
fn test_median_single_element() {
    assert_eq!(median!(42), 42);
}

#[test]
fn test_median_two_elements() {
    let numbers = vec![10, 20];
    assert_eq!(median!(10, 20), 15.0);
}

#[test]
fn test_median_three_elements() {
    let numbers = vec![10, 20, 30];
    assert_eq!(median!(10, 20, 30), 20);
}

#[test]
fn test_median_large_odd_elements() {
    let numbers: Vec<u64> = (1..=999).collect();
    assert_eq!(median!(@numbers), 499);
}

#[test]
fn test_median_large_even_elements() {
    let numbers: Vec<u64> = (1..=1000).collect();
    assert_eq!(median!(@numbers), 499.5);
}



#[test]
fn test_median_large_with_duplicates() {
    let numbers: Vec<u64> = vec![5; 100];
    assert_eq!(median!(@numbers), 5);
}