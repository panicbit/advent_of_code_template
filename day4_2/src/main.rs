use std::collections::BTreeSet;

use aoc::aoc;
use utils::re;

#[aoc(2023, 4, 2)]
fn main(input: &str) -> usize {
    let mut card_stacks = input
        .lines()
        .map(Card::parse)
        .map(CardStack::new)
        .collect::<Vec<_>>();

    for i in 0..card_stacks.len() {
        let card_stack = &card_stacks[i];
        let copies = card_stack.copies;
        let num_matches = card_stack.card.num_matches();

        for i in i + 1..i + 1 + num_matches {
            let Some(card_stack) = card_stacks.get_mut(i) else {
                break;
            };

            card_stack.copies += copies;
        }
    }

    card_stacks.iter().map(|stack| stack.copies).sum()
}

struct CardStack {
    card: Card,
    copies: usize,
}

impl CardStack {
    fn new(card: Card) -> Self {
        Self { card, copies: 1 }
    }
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

    fn num_matches(&self) -> usize {
        self.matches().count()
    }

    fn matches(&self) -> impl Iterator<Item = u32> + '_ {
        self.having.intersection(&self.winning).copied()
    }
}
