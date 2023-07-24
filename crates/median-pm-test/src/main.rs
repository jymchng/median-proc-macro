use median_pm_core::median;

fn main() {
    let median = median!(10, 20);
    let median_two = median!(1, 2, 3, 4, 5, 6);
    let median_three = median!(15, 56, 23, 56, 78);
    // assert_eq!(median!(10, 20), 5);
    println!("{:?} {:?} {:?}", median, median_two, median_three);
}
