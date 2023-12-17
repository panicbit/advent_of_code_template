use std::collections::BTreeSet;

use aoc::aoc;
use utils::re;

#[aoc(2023, 4, 1)]
fn main(input: &str) -> u32 {
    input
        .lines()
        .map(Card::parse)
        .map(|card| card.score())
        .sum()
}

struct Card {
    winning: BTreeSet<u32>,
    having: BTreeSet<u32>,
}

impl Card {
    fn parse(s: &str) -> Self {
        let card_re = re!(r"Card\s+\d+: (.*)");
        let captures = card_re.captures(s).unwrap();
        let mut number_parts = captures[1].split(" | ");
        let winning = number_parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|number| number.parse::<u32>().unwrap())
            .collect::<BTreeSet<_>>();
        let having = number_parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|number| number.parse::<u32>().unwrap())
            .collect::<BTreeSet<_>>();

        Card { winning, having }
    }

    fn matches(&self) -> impl Iterator<Item = u32> + '_ {
        self.having.intersection(&self.winning).copied()
    }

    fn score(&self) -> u32 {
        self.matches().fold(0, |score, _| match score {
            0 => 1,
            _ => score * 2,
        })
    }
}
