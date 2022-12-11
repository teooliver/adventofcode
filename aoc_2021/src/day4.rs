pub fn part_a(input: &str) {
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let draw_numbers = lines[0];

    let boards = &lines[1..];

    println!("Numbers {:?} \n Boards {:?}", draw_numbers, boards);
}

pub fn part_b(input: &str) {}

// #[cfg(test)]
// mod tests {
//     use super::{part_a, part_b};

//     #[test]
//     fn test_day_3() {
//         assert_eq!(part_a(include_str!("day4_input.txt")), 845186);
//         assert_eq!(part_b(include_str!("day4_input.txt")), 4636702);
//     }
// }
