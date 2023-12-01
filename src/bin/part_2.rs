use std::fs::read_to_string;

fn main() {
    let calibration_values: usize = read_to_string("./calibration_values.txt") 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|value| calculate_calibration_value(value))
        .sum();

    println!("Result is {:?}", calibration_values);
}

#[derive(Clone, Debug)]
struct NumberAsString {
    written: String,
    number: String
}

fn normalize_calibration_value_input(value: &str) -> String {
    let mut normalized_value = value.clone().to_string();
    let numbers_as_string: Vec<NumberAsString> = vec![
        NumberAsString { number: "o1e".to_owned(), written: "one".to_owned()},
        NumberAsString { number: "t2o".to_owned(), written: "two".to_owned()},
        NumberAsString { number: "t3e".to_owned(), written: "three".to_owned()},
        NumberAsString { number: "f4r".to_owned(), written: "four".to_owned()},
        NumberAsString { number: "f5e".to_owned(), written: "five".to_owned()},
        NumberAsString { number: "s6x".to_owned(), written: "six".to_owned()},
        NumberAsString { number: "s7n".to_owned(), written: "seven".to_owned()},
        NumberAsString { number: "e8t".to_owned(), written: "eight".to_owned()},
        NumberAsString { number: "n9e".to_owned(), written: "nine".to_owned()}
    ];

    numbers_as_string.into_iter().for_each(|number_as_string| {
        normalized_value = normalized_value.replace(&number_as_string.written, &number_as_string.number.to_string());
    });

    return normalized_value;
}

fn calculate_calibration_value(value: &str)-> usize {
    let normalized_value = normalize_calibration_value_input(value);

    let first_number = normalized_value.chars().into_iter().find(|val| val.to_digit(10).is_some()).unwrap();
    let last_number = normalized_value.chars().into_iter().rev().find(|val| val.to_digit(10).is_some()).unwrap();

    return format!("{first_number}{last_number}").parse::<usize>().unwrap();
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculations() {
        assert_eq!(calculate_calibration_value("two1nine"), 29);
        assert_eq!(calculate_calibration_value("eightwothree"), 83);
        assert_eq!(calculate_calibration_value("abcone2threexyz"), 13);
        assert_eq!(calculate_calibration_value("xtwone3four"), 24);
        assert_eq!(calculate_calibration_value("4nineeightseven2"), 42);
        assert_eq!(calculate_calibration_value("zoneight234"), 14);
        assert_eq!(calculate_calibration_value("7pqrstsixteen"), 76);
    }
}