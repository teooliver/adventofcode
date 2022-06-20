pub fn part_a(input: &str) -> i64 {
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let mut values_array: Vec<usize> = vec![0; 12];

    let mut gama_rate_array: Vec<u32> = vec![0; 12];
    let mut epsilon_rate_array: Vec<u32> = vec![0; 12];

    for line in lines.iter() {
        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                values_array[i] += 1;
            }
        }
    }

    for (i, value) in values_array.iter().enumerate() {
        let x: usize = lines.len().clone() / 2;
        if value > &x {
            gama_rate_array[i] = 1;
            epsilon_rate_array[i] = 0;
        } else {
            gama_rate_array[i] = 0;
            epsilon_rate_array[i] = 1;
        }
    }

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

// Alternative solution pushin values to a String instead of
// an Vec and then transforming it to a String.
pub fn part_a_alternative(input: &str) -> i64 {
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let mut values_array: Vec<u32> = vec![0; 12];

    let mut gama_rate_array = String::new();
    let mut epsilon_rate_array = String::new();

    for line in lines.iter() {
        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                values_array[i] += 1;
            }
        }
    }

    for (_i, value) in values_array.iter().enumerate() {
        let x: u32 = (lines.len().clone() / 2).try_into().unwrap();
        if value > &x {
            gama_rate_array.push('1');
            epsilon_rate_array.push('0');
        } else {
            gama_rate_array.push('0');
            epsilon_rate_array.push('1');
        }
    }

    let gama = i64::from_str_radix(&gama_rate_array, 2).unwrap();
    let epsilon = i64::from_str_radix(&epsilon_rate_array, 2).unwrap();

    gama * epsilon
}

fn get_element(input: &str, f: fn(usize, usize) -> bool) -> usize {
    // For each digit, add the most common one, then filter the lines
    // progressively against each push:
    //  "0"
    //  "01"
    //  "010"
    //  "0101"
    //  "01011"
    //  "010110"
    //  "0101101"
    //  "01011011"
    //  "010110110"
    //  "0101101100"
    //  "01011011001"
    //  "010110110011"

    let mut most_common_bits = String::new();

    for digit in 0..12 {
        let filtered: Vec<_> = input
            .lines()
            .filter(|line| line.starts_with(&most_common_bits))
            .collect();

        if filtered.len() == 1 {
            break;
        }

        let count1 = filtered
            .iter()
            .map(|&line| line.chars().nth(digit).unwrap())
            .filter(|&c| c == '1')
            .count();

        // The function checks for the most or least common bits
        most_common_bits.push(if f(count1, filtered.len() - count1) {
            '1'
        } else {
            '0'
        });
    }

    let filtered: Vec<_> = input
        .lines()
        .filter(|line| line.starts_with(&most_common_bits))
        .collect();

    usize::from_str_radix(filtered[0], 2).unwrap()
}

pub fn part_b(input: &str) -> usize {
    let oxygen = get_element(input, |count1, count0| count1 >= count0);
    let co2 = get_element(input, |count1, count0| count1 < count0);
    oxygen * co2
}

#[cfg(test)]
mod tests {
    use crate::day3::{part_a_alternative, part_b};

    use super::part_a;
    #[test]
    fn test_day_3() {
        assert_eq!(part_a(include_str!("day3_input.txt")), 845186);
        assert_eq!(part_a_alternative(include_str!("day3_input.txt")), 845186);
        assert_eq!(part_b(include_str!("day3_input.txt")), 4636702);
    }
}
