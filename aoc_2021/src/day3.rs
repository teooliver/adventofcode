pub fn part_a(input: &str) -> i64 {
    // let gamma_rate = "000000000000";
    // let epsilon_rate = "111111111111";

    let lines: Vec<&str> = input.trim().split('\n').collect();

    let mut values_array: Vec<u32> = vec![0; 12];

    //This could be a String
    let mut gama_rate_array: Vec<u32> = vec![0; 12];
    let mut epsilon_rate_array: Vec<u32> = vec![0; 12];

    for line in lines.iter() {
        // let mut amount_of_ones = 0;
        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                values_array[i] += 1;
                // amount_of_ones += 1
            }
        }
    }

    for (i, value) in values_array.iter().enumerate() {
        let x: u32 = (lines.len().clone() / 2).try_into().unwrap();
        if value > &x {
            gama_rate_array[i] = 1;
            epsilon_rate_array[i] = 0;
        } else {
            gama_rate_array[i] = 0;
            epsilon_rate_array[i] = 1;
        }
    }

    // println!("{:?}", gama_rate_array);
    // println!("{:?}", epsilon_rate_array);

    let joined_gama_string: String = gama_rate_array.iter().map(|num| num.to_string()).collect();
    let gama_rate: String = format!("{}", joined_gama_string);

    let joined_epsilon_string: String = epsilon_rate_array
        .iter()
        .map(|num| num.to_string())
        .collect();
    let epsilon_rate: String = format!("{}", joined_epsilon_string);

    let gama = i64::from_str_radix(&gama_rate, 2).unwrap();
    let epsilon = i64::from_str_radix(&epsilon_rate, 2).unwrap();

    gama * epsilon
}

// pub fn part_b(input: &str) -> i64 {}

#[cfg(test)]
mod tests {
    use super::part_a;
    #[test]
    fn test_day_3() {
        assert_eq!(part_a(include_str!("day3_input.txt")), 845186);
        // assert_eq!(part_b(include_str!("day3_input.txt")), 1954293920);
    }
}
