

fn part_a(input: &str) -> String {
    let calories = input
        .split("\n\n")
        .map(|cal_block| {
            cal_block
            .lines()
            .map(|item| item.parse::<u32>().unwrap())
            .sum::<u32>()
        })
        .max()
        .unwrap();

    calories.to_string()
}

fn part_b(input: &str) -> String {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|cal_block| {
            cal_block
            .lines()
            .map(|item| item.parse::<u32>().unwrap())
            .sum::<u32>()
        }).collect();

    calories.sort_by(|a,b| b.cmp(a));

    let sum_top_three: u32 = calories.iter().take(3).sum();

    sum_top_three.to_string()
}

#[cfg(test)]
mod tests {
    use super::{part_a, part_b};
    #[test]
    fn test_day_1() {
        assert_eq!(part_a(include_str!("day1_input.txt")), "70720");
        assert_eq!(part_b(include_str!("day1_input.txt")), "207148");
    }
}
