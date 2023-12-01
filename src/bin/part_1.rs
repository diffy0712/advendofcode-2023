use std::fs::read_to_string;

fn main() {
    let calibration_values: usize = read_to_string("./calibration_values.txt") 
        .unwrap()  
        .lines()
        .filter(|line| !line.is_empty()) 
        .map(|value| calculate_calibration_value(value))
        .sum();

    println!("Result is {:?}", calibration_values);
}


fn calculate_calibration_value(value: &str)-> usize {
    let first_number = value.chars().into_iter().find(|val| val.to_digit(10).is_some()).unwrap();
    let last_number = value.chars().into_iter().rev().find(|val| val.to_digit(10).is_some()).unwrap();

    return format!("{first_number}{last_number}").parse::<usize>().unwrap();
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculations() {
        assert_eq!(calculate_calibration_value("1abc2"), 12);
        assert_eq!(calculate_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(calculate_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(calculate_calibration_value("treb7uchet"), 77);
    }
}