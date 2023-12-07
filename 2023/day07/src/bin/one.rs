use std::{io::BufRead, collections::{HashMap, hash_map::RandomState}};
use std::hash::Hash;

fn histogram<T>(from: &mut dyn Iterator< Item = T>) -> HashMap<T, i64, RandomState> 
where T: Eq + Hash + Clone
{
    let mut result = HashMap::<T, i64, RandomState>::new();
    for val in from {
        result.insert(val.clone(), result.get(&val).unwrap_or(&0) + 1);
    };
    result
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[allow(dead_code)]
enum PokerHand {
    #[default]
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
#[derive(Clone, Ord, Debug, Default, PartialEq, Eq, Hash)]
struct Hand {
    pub cards: String,
    pub bid: i64,
    pub value: PokerHand

}
fn card_cmp(left: char, right: char) -> Option<std::cmp::Ordering> {
    let card_order = "23456789TJQKA";
    card_order.find(left).unwrap().partial_cmp(&card_order.find(right).unwrap())
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.value.partial_cmp(&other.value) {
            Some(core::cmp::Ordering::Equal) => Some(
                self
                .cards
                .chars()
                .zip(
                    other
                    .cards
                    .chars())
                .flat_map(|(left, right)| card_cmp(left, right))
                .find(|ordering| *ordering != core::cmp::Ordering::Equal)
                .unwrap_or(core::cmp::Ordering::Equal)
            ),
            ord => ord,
        }
    }
}
impl Hand {
    pub fn new(cards: String, bid: i64) -> Hand {
        let binding = histogram(&mut cards.chars());
        let frequencies = histogram(&mut binding.values()); 
        Hand { cards, bid, value: 
            if *frequencies.get(&5).unwrap_or(&0) == 1 {
                PokerHand::FiveOfAKind
            } else if *frequencies.get(&4).unwrap_or(&0) == 1 {
                PokerHand::FourOfAKind
            } else if *frequencies.get(&3).unwrap_or(&0) == 1 && *frequencies.get(&2).unwrap_or(&0) == 1 {
                PokerHand::FullHouse
            } else if *frequencies.get(&3).unwrap_or(&0) == 1 {
                PokerHand::ThreeOfAKind
            } else if *frequencies.get(&2).unwrap_or(&0) == 2 {
                PokerHand::TwoPair
            } else if *frequencies.get(&2).unwrap_or(&0) == 1 {
                PokerHand::Pair
            } else { PokerHand::HighCard }
         }
    }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input: Vec<Hand> = std::io::BufReader::new(std::io::stdin())
        .lines()
        .filter_map(std::io::Result::ok)
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            Hand::new( cards.to_string(), bid.parse::<i64>().unwrap() )
        })
        .collect();
    input.sort();
    println!("{:#?}", input.iter().enumerate().map(|(rank, hand)| (rank as i64 + 1 ) * hand.bid).sum::<i64>());
    Ok(())
}