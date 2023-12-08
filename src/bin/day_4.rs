use std::fs::read_to_string;

fn main() {
    let input_values: String = read_to_string("./scratchcards.txt").unwrap();

    let winning_numbers = input_values  
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| -> &str {
            let start_bytes = line.find(":").unwrap();
            let result = &line[start_bytes + 1..];

            return result;
        }) 
        .map(|line| line.split("|").collect::<Vec<&str>>())
        .map(|lines| -> (Vec<usize>, Vec<usize>) {
            let winning_numbers = lines.get(0).unwrap().trim().split(" ").filter(|item| !item.is_empty()).map(|number| number.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            let our_numbers = lines.get(1).unwrap().trim().split(" ").filter(|item| !item.is_empty()).map(|number| number.parse::<usize>().unwrap()).collect::<Vec<usize>>();

            return (winning_numbers, our_numbers);
        })
        .map(|(winning_numbers, our_numbers)| winning_numbers.into_iter().filter(|winning_number| our_numbers.contains(&winning_number)).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    println!("Points in total: {:?}", calculate_points(&winning_numbers));
    println!("Scratchpads in total: {:?}", calculate_scratchpads(&winning_numbers));
}

fn calculate_points(winning_numbers: &Vec<Vec<usize>>) -> usize {
    return winning_numbers.into_iter().map(|winning_numbers| -> usize {
        let number_of_winning_numbers = winning_numbers.len();
        match number_of_winning_numbers {
            0 => return 0,
            1 => return 1,
            _ => return 2_usize.pow((number_of_winning_numbers - 1).try_into().unwrap())
        };
    })
    .sum();
}

fn calculate_scratchpads(scratchpad_winning_numbers: &Vec<Vec<usize>>) -> usize {
    let mut scratchpads_count = scratchpad_winning_numbers.into_iter().map(|_| 1).collect::<Vec<usize>>();

    scratchpad_winning_numbers.into_iter().enumerate().for_each(|(index, item)| -> () {
        let scratchpad_count = scratchpads_count.get(index).unwrap();
        for _ in 0..*scratchpad_count {
            for n in 1..item.len()+1 {
                let element = scratchpads_count.get_mut(index + n).unwrap();
                *element+=1;
            }
        }
    });

    return scratchpads_count.iter().sum();
}