use std::fs::read_to_string;

fn main() {
    let races_input = read_to_string("./races.txt") .unwrap();

    process(&races_input);

    let fixed_race_input = races_input.replace(" ", "").replace(":", ": ");
    process(&fixed_race_input);
}

fn process(races_input: &str) {
    let races = races_input.lines()
        .map(|line| line.split(" ").skip(1).filter(|line_part| !line_part.is_empty()).map(|line_part| line_part.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let times = races.get(0).expect("Times should exist");
    let distances = races.get(1).expect("Distances should exust");

    let races = times.iter().enumerate().map(|(index, time)| Race {
        time: time.clone(),
        distance: distances.get(index).expect("Distance should exist").to_owned()
    }).collect::<Vec<Race>>();

    let product: usize = races.iter().map(|race| race.get_winning_times_count()).product();

    println!("What do you get if you multiply winning numbers together?: {:?}", product);
}


#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize
}

impl Race {
    fn get_winning_times_count(&self) -> usize {
        let first_winner_time = (0..self.time).into_iter().find(|change_time| -> bool {
            let distance_travelled = change_time * (self.time - change_time);

            return distance_travelled > self.distance;
        }).unwrap();

        let number_of_winnin_ways = (self.time + 1) - (2  * first_winner_time);

        return number_of_winnin_ways;
    }
}
