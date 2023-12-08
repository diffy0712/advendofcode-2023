use std::fs::read_to_string;

fn main() {
    let input_values: String = read_to_string("./game_data.txt").unwrap();
    let test_set = GameSet{red: 12, blue: 14, green: 13};

    let games = parse_games_from_input(&input_values);

    println!("Sum of possible game ids: {:}", get_sum_of_possible_game_ids(&games, &test_set));
    println!("Sum of power of minimum game sets: {:}", get_sum_of_power_of_sets(&games));
}

fn get_sum_of_possible_game_ids(games: &Vec<Game>, test_set: &GameSet) -> usize {
    return games.iter()
        .filter(|game| game.is_possible_with_set(test_set))
        .map(|game| game.id)
        .sum();
}

fn get_sum_of_power_of_sets(games: &Vec<Game>) -> usize {
    return games.iter()
        .map(|game| game.get_minimum_set())
        .map(|minimum_game_set|minimum_game_set.blue * minimum_game_set.green * minimum_game_set.red)
        .sum();
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<GameSet>
}

#[derive(Debug)]
struct GameSet {
    red: usize,
    green: usize,
    blue: usize
}

impl Game {
    pub fn is_possible_with_set(&self, test_set: &GameSet) -> bool {
        for set in self.sets.iter() {
            if set.blue > test_set.blue || set.green > test_set.green || set.red > test_set.red {
                return false;
            }
        }

        return true;
    }

    pub fn get_minimum_set(&self) -> GameSet {
        return GameSet {
            red: self.sets.iter().map(|set| set.red).max().unwrap(), 
            green: self.sets.iter().map(|set| set.green).max().unwrap(), 
            blue: self.sets.iter().map(|set| set.blue).max().unwrap() 
        }
    }
}

fn parse_games_from_input(input: &String) -> Vec<Game> {
    return input  
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_game(line))
        .collect();
}

fn parse_game(game_data_line: &str) -> Game {
    let start_bytes = game_data_line.find(":").unwrap();
    let sets_line = &game_data_line[start_bytes + 1..];
    let sets: Vec<GameSet> = sets_line.split(";").map(|set_line| parse_game_set(set_line)).collect();

    return Game {
        id: parse_game_id(game_data_line),
        sets
    };
}

fn parse_game_id(game_data_line: &str)-> usize {
    let end_bytes = game_data_line.find(":").unwrap();
    let result = &game_data_line[5..end_bytes];

    return result.parse::<usize>().unwrap();
}

fn parse_game_set(game_data_line: &str)-> GameSet {
    let parts = game_data_line.split(",").map(|part| part.trim()).collect::<Vec<&str>>();

    let red = match parts.iter().find(|part| part.contains("red")) {
        Some(part) => parse_color_amount(part),
        None => 0
    };
    let green = match parts.iter().find(|part| part.contains("green")) {
        Some(part) => parse_color_amount(part),
        None => 0
    };
    let blue = match parts.iter().find(|part| part.contains("blue")) {
        Some(part) => parse_color_amount(part),
        None => 0
    };

    return GameSet { red, green, blue };
}

fn parse_color_amount(line: &str) -> usize {
    let end_bytes = line.find(" ").unwrap();
    let result = &line[0..end_bytes];

    return result.parse::<usize>().unwrap();
}
