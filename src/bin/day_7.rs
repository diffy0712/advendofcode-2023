use std::{fs::read_to_string, cmp::Ordering, collections::HashMap};

fn main() {
    let hands_input = read_to_string("./hands.txt") .unwrap();

    println!("Hands total: {:?}", process(&hands_input, false));
    println!("Hands total with jokers: {:?}", process(&hands_input, true));
}

fn process(hands_input: &String, jokers: bool) -> usize {
    let mut hands = parse_hands(&hands_input, jokers);

    hands.sort_by(|hand1, hand2| -> Ordering {
        if hand1.hand_type == hand2.hand_type {
            for (index, card) in hand1.cards.iter().enumerate() {
                let card2 = hand2.cards.get(index).expect("error");
                if card == card2 {
                    continue;
                }

                return match card > card2 {
                    true => Ordering::Greater,
                    false => Ordering::Less,
                }
            }
        }

        if hand1.hand_type < hand2.hand_type {
            return Ordering::Greater;
        }

        return Ordering::Less;
    });

    return hands.iter().enumerate().map(|(rank, hand)| hand.bid * (rank + 1)).sum();
}

fn parse_hands(hands_input: &String, jokers: bool) -> Vec<Hand> {
    return hands_input.lines().into_iter().filter(|line| !line.is_empty()).map(|line| -> Hand {
        let (cards, bid) = sscanf::scanf!(line, "{} {}", &str, usize).expect("Input line not valid");

        let cards = cards.chars().map(|card|
            match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => match jokers {
                    true => 1,
                    false => 11,
                },
                'T' => 10,
                card => card.to_digit(10).expect("card should be a number"),
        }).map(|card_number| usize::try_from(card_number).expect("card should be a number"))
        .collect::<Vec<usize>>();

        let hand_type = get_hand_type(&cards, jokers);

        return Hand {
            bid,
            cards,
            hand_type
        };
    }).collect::<Vec<Hand>>();
}

fn get_hand_type(cards: &Vec<usize>, jokers: bool) -> HandType {
    let mut kinds = cards.iter()
        .fold(HashMap::new(), |mut map, card| -> HashMap<usize, u8> {
            let char_count = map.get(card).unwrap_or(&0_u8);
            map.insert(card.clone(), char_count + 1);

            return map;
        });
    

    if jokers {
        let number_of_jokers = kinds.get(&1_usize).unwrap_or(&0_u8).to_owned();

        match number_of_jokers {
            1..=4 => {
                kinds.remove(&1_usize);
                let highest = kinds.iter().max_by_key(|entry | entry.1).unwrap();
                kinds.insert(*highest.0, highest.1 + number_of_jokers);
            }
            _ => (),
        }
    }

    let result = match kinds.len() {
        1 => HandType::FiveOfAKind,
        2 => match kinds.values().find(|value| **value == 4_u8) {
            Some(_) => HandType::FourOfAKind,
            None => HandType::FullHouse,
        },
        3 => match kinds.values().find(|value| **value == 3_u8) {
            Some(_) => HandType::ThreeOfAKind,
            None => HandType::TwoPair,
        },
        4 => HandType::OnePair,
        _ => HandType::HighCard,
    };


    return result;
}

#[derive(Debug)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
    hand_type: HandType
}

#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}