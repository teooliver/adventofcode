fn part_a(input: &str) -> i64 {
    let report: Vec<i64> = input
        .trim()
        .split('\n')
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut depth_increments = 0;

    let mut previous_number = &report[0];

    for number in report.iter() {
        if number > previous_number {
            depth_increments += 1;
        }
        previous_number = number;
    }

    return depth_increments;
}

pub fn part_b(input: &str) -> i64 {
    let report: Vec<i64> = input
        .trim()
        .split('\n')
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut depth_increments = 0;

    for i in 3..report.len() {
        let current_window = report[i] + report[i - 1] + report[i - 2];
        let previous_window = report[i - 1] + report[i - 2] + report[i - 3];

        if current_window > previous_window {
            depth_increments += 1;
        }
    }

    depth_increments
}

#[cfg(test)]
mod day_one_tests {
    use super::{part_a, part_b};
    #[test]
    fn test_day_one() {
        assert_eq!(part_a(include_str!("day1_input.txt")), 1226);
        assert_eq!(part_b(include_str!("day1_input.txt")), 1252);
    }
}
