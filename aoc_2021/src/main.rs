mod day1;
mod day2;
mod day3;

fn main() {
    fn playground() {
        let a: Vec<&str> = vec!["0", "1", "2", "3", "4", "5", "6"];
        let b: Vec<&str> = vec!["0", "4", "5", "6"];

        let mut iter: Vec<_> = a.iter().filter(|x| b.contains(x)).collect(); // both & and *

        println!("=====> {:?}", iter);
    }
    // playground();

    // day1::part_b(include_str!("day1_input.txt"));
    // day2::part_b(include_str!("day2_input.txt"));
    // day3::part_a(include_str!("day3_input.txt"));
    day3::part_b(include_str!("day3_input.txt"));
}
