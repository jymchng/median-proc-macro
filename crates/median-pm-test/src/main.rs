use median_pm_core::median;

fn main() {
    let median = median!(1, 2, 3, 4, 5);
    let median_two = median!(1, 2, 3, 4, 5, 6);
    let median_three = median!(5, 56, 23, 56, 78);
    println!("{:?} {:?} {:?}", median, median_two, median_three);
}