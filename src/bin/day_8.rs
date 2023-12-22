use std::fs::read_to_string;

fn main() {
    let network_input = read_to_string("./network.txt") .unwrap();

    let (sequence, nodes) = parse_network(&network_input);

    

    //println!("iteration count part 1: {:?}", part_1(sequence, &nodes));
    println!("iteration count part 2: {:?}", part_2(sequence, &nodes));
}

fn part_2(sequence: &str, nodes: &Vec<(&str, &str, &str)>) -> usize { 
    let mut iteration_count = 0;
    let mut sequence_number = 0;
    let mut current_nodes = nodes.iter().filter(|node| node.0.ends_with("A")).collect::<Vec<&(&str, &str, &str)>>();
    
    loop {
        iteration_count+=1;

        let next_side = sequence.chars().nth(sequence_number).expect("sequence index not found");
        current_nodes.clone().into_iter().enumerate().for_each(|(index,node)| -> () {
            let next_node_name = if next_side == 'L' {
                node.1
            } else {
                node.2
            };

            let next_node = nodes.iter().find(|node| node.0 == next_node_name ).unwrap();
            current_nodes[index] = next_node;
        });

        let ending_nodes_length = current_nodes.iter().filter(|node| node.0.ends_with("Z")).count();
        if ending_nodes_length == current_nodes.len() {
            break;
        }

        sequence_number += 1;
        if sequence_number >= sequence.len() {
            sequence_number = 0;
        }
    }

    return iteration_count;
}

fn part_1(sequence: &str, nodes: &Vec<(&str, &str, &str)>) -> usize {
    let mut iteration_count = 0;
    let mut sequence_number = 0;
    let mut node_index = nodes.iter().enumerate().find(|(_, node)| node.0 == "AAA" ).unwrap().0;
    loop {
        iteration_count+=1;

        let next_side = sequence.chars().nth(sequence_number).expect("sequence index not found");
        let node = nodes.get(node_index).expect("node should exist");

        let next_node_name = if next_side == 'L' {
            node.1
        } else {
            node.2
        };

        let next_node = nodes.iter().enumerate().find(|(_, node)| node.0 == next_node_name ).unwrap();

        if next_node.1.0 == "ZZZ" {
            break;
        }

        node_index = next_node.0;
        sequence_number += 1;
        if sequence_number >= sequence.len() {
            sequence_number = 0;
        }
    }

    return iteration_count;
}

fn parse_network(network_input: &str) -> (&str, Vec<(&str, &str, &str)>) {
    let lines = network_input.lines().collect::<Vec<&str>>();

    let sequence = lines.first().expect("unable to parse sequence").clone();
    let nodes = lines.into_iter()
        .skip(2)
        .map(|node| sscanf::scanf!(node, "{} = ({}, {})", &str, &str, &str).expect("Unable to parse input line"))
        .collect::<Vec<(&str, &str, &str)>>();

    return (sequence, nodes);
}