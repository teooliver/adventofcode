pub struct EndPosition {
    x: i64,
    y: i64,
}

pub fn part_a(input: &str) -> i64 {
    let mut end_position = EndPosition { x: 0, y: 0 };

    let lines: Vec<&str> = input.trim().split('\n').collect();

    for line in lines {
        let direction: Vec<&str> = line.split(" ").collect();

        match direction[0] {
            "forward" => {
                end_position.x += direction[1].parse::<i64>().unwrap();
            }

            "down" => {
                end_position.y += direction[1].parse::<i64>().unwrap();
            }

            "up" => {
                end_position.y -= direction[1].parse::<i64>().unwrap();
            }
            _ => panic!(),
        }
    }

    end_position.x * end_position.y
}

pub fn part_b(input: &str) -> i64 {
    let mut end_position = EndPosition { x: 0, y: 0 };
    let mut aim = 0;

    let lines: Vec<&str> = input.trim().split('\n').collect();

    for line in lines {
        let direction: Vec<&str> = line.split(" ").collect();

        match direction[0] {
            "forward" => {
                end_position.x += direction[1].parse::<i64>().unwrap();
                end_position.y += aim * direction[1].parse::<i64>().unwrap();
            }

            "down" => {
                aim += direction[1].parse::<i64>().unwrap();
            }

            "up" => {
                aim -= direction[1].parse::<i64>().unwrap();
            }
            _ => panic!(),
        }
    }

    end_position.x * end_position.y
}

#[cfg(test)]
mod tests {
    use super::{part_a, part_b};
    #[test]
    fn test_day_2() {
        assert_eq!(part_a(include_str!("day2_input.txt")), 1670340);
        assert_eq!(part_b(include_str!("day2_input.txt")), 1954293920);
    }
}
