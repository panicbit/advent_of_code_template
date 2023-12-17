use std::collections::{BTreeMap, HashSet};

use aoc::aoc;
use itertools::Itertools;

#[aoc(2023, 3, 1)]
fn main(input: &str) -> u32 {
    let blueprint = Blueprint::parse(input);

    blueprint
        .parts()
        .flat_map(|part| part.numbers.into_iter().map(|(_, number)| number))
        .sum()
}

struct Blueprint {
    data: BTreeMap<Pos, char>,
}

impl Blueprint {
    fn parse(s: &str) -> Self {
        let data = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, ch)| {
                        let pos = Pos::new(x as i32, y as i32);

                        (pos, ch)
                    })
                    .filter(|(_, ch)| *ch != '.')
            })
            .collect::<BTreeMap<_, _>>();

        Self { data }
    }

    fn parts(&self) -> impl Iterator<Item = Part> + '_ {
        self.symbols().map(|(pos, symbol)| {
            let mut part = Part::new(*pos, *symbol);

            for (pos, number) in self.adjacent_numbers(*pos) {
                part.add_number(pos, number);
            }

            part
        })
    }

    fn get(&self, pos: Pos) -> Option<char> {
        self.data.get(&pos).cloned()
    }

    fn symbols(&self) -> impl Iterator<Item = (&Pos, &char)> + '_ {
        self.data
            .iter()
            .filter(|(_, char)| char.is_ascii_punctuation())
    }

    fn adjacents(&self, pos: Pos) -> impl Iterator<Item = (Pos, char)> + '_ {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|delta| *delta != (0, 0))
            .map(move |(dx, dy)| Pos {
                x: pos.x + dx,
                y: pos.y + dy,
            })
            .flat_map(|adjacent_pos| self.get(adjacent_pos).map(|ch| (adjacent_pos, ch)))
    }

    fn adjacent_numbers(&self, pos: Pos) -> impl Iterator<Item = (Pos, u32)> + '_ {
        self.adjacent_digits(pos).map(|(pos, _)| {
            let start = self.find_number_start(pos);
            let number = self.read_number_at(start);

            (start, number)
        })
    }

    fn adjacent_digits(&self, pos: Pos) -> impl Iterator<Item = (Pos, char)> + '_ {
        self.adjacents(pos).filter(|(_, ch)| ch.is_ascii_digit())
    }

    fn find_number_start(&self, mut pos: Pos) -> Pos {
        loop {
            let new_pos = pos.left();

            let Some(ch) = self.get(pos.left()) else {
                break;
            };

            if !ch.is_ascii_digit() {
                break;
            }

            pos = new_pos;
        }

        pos
    }

    fn read_number_at(&self, mut pos: Pos) -> u32 {
        let mut number = String::new();

        loop {
            let Some(ch) = self.get(pos) else {
                break;
            };

            if !ch.is_ascii_digit() {
                break;
            }

            number.push(ch);

            pos = pos.right();
        }

        number.parse().unwrap()
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}

struct Part {
    pos: Pos,
    symbol: char,
    numbers: HashSet<(Pos, u32)>,
}

impl Part {
    fn new(pos: Pos, symbol: char) -> Self {
        Self {
            pos,
            symbol,
            numbers: HashSet::new(),
        }
    }

    fn add_number(&mut self, pos: Pos, number: u32) {
        self.numbers.insert((pos, number));
    }
}
