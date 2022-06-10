fn main() {
    let report: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    let mut depth_increments = 0;

    let mut previous_number = &report[0];

    for number in report.iter() {
        if number > previous_number {
            depth_increments += 1;
        }
        previous_number = number;
    }

    println!("How many measurements are larger than the previous measurement?");
    println!("{}", depth_increments);
}
