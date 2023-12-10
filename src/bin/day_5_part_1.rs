use std::{fs::read_to_string, ops::Range, usize};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IntoParallelIterator};

fn main() {
    let almanac_input = read_to_string("./almanac.txt") .unwrap();

    let almanac_parts = almanac_input.split("\n\n").collect::<Vec<&str>>();
    let seeds = parse_seeds(&almanac_parts);
    let maps = parse_map(&almanac_parts);

    println!("lowest location: {:?}", find_lowest_location(&maps, seeds));
}

#[derive(Debug)]
struct Map (Vec<(Range<usize>, Range<usize>)>);

impl Map {
    fn translate(&self, source: usize) -> usize {
        let map = self.0.par_iter().find_first(|map| map.1.contains(&source));

        let Some((destination_range, source_range)) = map
        else {
            return source;
        };

        let diff = source - source_range.start;

        return destination_range.start + diff;
    }
}

fn find_lowest_location(maps: &Vec<Map>, seeds: Vec<usize>) -> usize {
    let locations = seeds.into_par_iter()
        .map(|seed| {
            maps.iter()
                .fold(seed, |seed, map| map.translate(seed))
        }).collect::<Vec<usize>>();

    return locations.iter().min().unwrap().to_owned();
}

fn parse_map(almanac_parts: &Vec<&str>) -> Vec<Map> {
    almanac_parts.iter()
        .skip(1)
        .map(|map_part| map_part.lines()
            .skip(1)
            .map(|map_line| -> (Range<usize>, Range<usize>) {
                let (destination, source, length) = sscanf::sscanf!(map_line, "{} {} {}", usize, usize, usize).expect("map line failed to parse");
                return (destination..(destination + length), source..(source + length));
            })
            .collect::<Vec<(Range<usize>, Range<usize>)>>()
        )
        .map(|item| Map(item))
        .collect::<Vec<Map>>()
}

fn parse_seeds(almanac_parts: &Vec<&str>) -> Vec<usize>{
    almanac_parts.first()
        .map(|seed_part| seed_part
            .split(" ")
            .skip(1)
            .map(|seed_str| seed_str.parse::<usize>().unwrap())
            .collect::<Vec<usize>>())
        .expect("Seeds failed to parse")
}