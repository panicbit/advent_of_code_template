use aoc::aoc;
use utils::re;

#[aoc(2023, 2, 1)]
fn main(input: &str) -> u32 {
    let games = input.lines().map(parse_game);
    let bag = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    games
        .filter(|game| game.possible_with_bag(&bag))
        .map(|game| game.id)
        .sum()
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn possible_with_bag(&self, bag: &Draw) -> bool {
        let max_draw = self.max_draw();

        if max_draw.red > bag.red {
            return false;
        }

        if max_draw.green > bag.green {
            return false;
        }

        if max_draw.blue > bag.blue {
            return false;
        }

        true
    }

    fn max_draw(&self) -> Draw {
        let mut max_draw = Draw::default();

        for draw in &self.draws {
            max_draw = max_draw.max(draw);
        }

        max_draw
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl Draw {
    fn zip_with<F>(&self, other: &Self, f: F) -> Draw
    where
        F: Fn(u32, u32) -> u32,
    {
        Draw {
            red: f(self.red, other.red),
            green: f(self.green, other.green),
            blue: f(self.blue, other.blue),
        }
    }

    fn max(&self, other: &Self) -> Self {
        self.zip_with(other, |a, b| a.max(b))
    }
}

fn parse_game(s: &str) -> Game {
    let re = re!(r"^Game (?P<id>\d+): (?P<draws>.*)$");
    let caps = re.captures(s).unwrap();

    let id = caps["id"].parse::<u32>().unwrap();
    let draws = parse_draws(&caps["draws"]);

    Game { id, draws }
}

fn parse_draws(s: &str) -> Vec<Draw> {
    s.split("; ").map(parse_draw).collect()
}

fn parse_draw(s: &str) -> Draw {
    let mut draw = Draw::default();

    for color_set in s.split(", ") {
        let color_set = parse_color_set(color_set);

        match color_set.color {
            Color::Red => draw.red += color_set.amount,
            Color::Green => draw.green += color_set.amount,
            Color::Blue => draw.blue += color_set.amount,
        }
    }

    draw
}

fn parse_color_set(s: &str) -> ColorSet {
    let mut parts = s.split(' ');

    ColorSet {
        amount: parts.next().unwrap().parse::<u32>().unwrap(),
        color: parse_color(parts.next().unwrap()),
    }
}

fn parse_color(s: &str) -> Color {
    match s {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => unreachable!(),
    }
}

struct ColorSet {
    color: Color,
    amount: u32,
}

enum Color {
    Red,
    Green,
    Blue,
}
