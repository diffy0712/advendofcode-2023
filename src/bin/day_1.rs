use std::fs::read_to_string;

fn main() {
    let input_values = read_to_string("./calibration_values.txt") .unwrap();
    let part_1 = get_sum_of_calibrations(&input_values, false);

    println!("Result is {:?}", part_1);

    let part_2 = get_sum_of_calibrations(&input_values, true);
    println!("Result is {:?}", part_2);
}

fn get_sum_of_calibrations(input: &String, normalize: bool) -> u32 {
    let sum_of_calibration_values: u32 = input  
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| calculate_calibration_value(line, normalize))
        .sum();

    return sum_of_calibration_values;
}

fn calculate_calibration_value(line: &str, normalize: bool)-> u32 {
    let calibration_line = if normalize { 
        normalize_calibration_value_input(line)
    } else {
        line.to_string()
    };

    let numbers = calibration_line.chars()
        .into_iter()
        .filter_map(|val| val.to_digit(10))
        .collect::<Vec<u32>>();

    return 10 * numbers.first().unwrap() + numbers.last().unwrap();
}

fn normalize_calibration_value_input(value: &str) -> String {
    return value.to_string()
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculations() {
        assert_eq!(calculate_calibration_value("1abc2", false), 12);
        assert_eq!(calculate_calibration_value("pqr3stu8vwx", false), 38);
        assert_eq!(calculate_calibration_value("a1b2c3d4e5f", false), 15);
        assert_eq!(calculate_calibration_value("treb7uchet", false), 77);
    }

    #[test]
    fn test_normalized_calibrations() {
        assert_eq!(calculate_calibration_value("two1nine", true), 29);
        assert_eq!(calculate_calibration_value("eightwothree", true), 83);
        assert_eq!(calculate_calibration_value("abcone2threexyz", true), 13);
        assert_eq!(calculate_calibration_value("xtwone3four", true), 24);
        assert_eq!(calculate_calibration_value("4nineeightseven2", true), 42);
        assert_eq!(calculate_calibration_value("zoneight234", true), 14);
        assert_eq!(calculate_calibration_value("7pqrstsixteen", true), 76);
    }

    #[test]
    fn test_sum_of_calibrations() {
        let input_one = "1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet\n\
            ".to_string();
        assert_eq!(get_sum_of_calibrations(&input_one, false), 142);


        let input_one = "two1nine\n\
            eightwothree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen\n\
            ".to_string();
        assert_eq!(get_sum_of_calibrations(&input_one, true), 281);
    }
}