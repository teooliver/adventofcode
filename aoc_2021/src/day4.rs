pub fn part_a(input: &str) {}

pub fn part_b(input: &str) {}

#[cfg(test)]
mod tests {
    use crate::day4::{part_a_alternative, part_b};

    use super::part_a;
    #[test]
    fn test_day_3() {
        assert_eq!(part_a(include_str!("day4_input.txt")), 845186);
        assert_eq!(part_b(include_str!("day4_input.txt")), 4636702);
    }
}
