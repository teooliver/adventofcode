#[derive(Debug, PartialEq)]
pub struct EndPosition {
    x: i64,
    y: i64,
}

fn map_direction_to_struct(direction: Vec<&str>, mut end_position: EndPosition) {
    if direction[0] == "forward" {
        end_position.x = end_position.x + direction[1].parse::<i64>().unwrap();
    }

    if direction[0] == "down" {
        end_position.y = end_position.y - direction[1].parse::<i64>().unwrap();
    }

    if direction[0] == "up" {
        end_position.y = end_position.y + direction[1].parse::<i64>().unwrap();
    }
}

pub fn part_a(input: &str) -> i64 {
    let mut end_position = EndPosition { x: 0, y: 0 };

    let lines: Vec<&str> = input.trim().split('\n').collect();

    for line in lines {
        let direction: Vec<&str> = line.split(" ").collect();

        if direction[0] == "forward" {
            end_position.x = end_position.x + direction[1].parse::<i64>().unwrap();
        }

        if direction[0] == "down" {
            end_position.y = end_position.y + direction[1].parse::<i64>().unwrap();
        }

        if direction[0] == "up" {
            end_position.y = end_position.y - direction[1].parse::<i64>().unwrap();
        }
        // map_direction_to_struct(line_vec, end_destination)
    }

    end_position.x * end_position.y
}

#[cfg(test)]
mod day_one_tests {
    use super::part_a;
    #[test]
    fn test_day_one() {
        assert_eq!(part_a(include_str!("day2_input.txt")), 1670340);
    }
}
