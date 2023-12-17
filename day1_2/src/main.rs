use aoc::aoc;

#[aoc(2023, 1, 2)]
fn main(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = parse_digits(line).collect::<Vec<_>>();
            let number = 10 * digits[0] + digits[digits.len() - 1];

            number
        })
        .sum::<u32>()
}

fn parse_digits(digit: &str) -> impl Iterator<Item = u32> + '_ {
    (0..digit.len())
        .map(|start| &digit[start..])
        .filter_map(parse_digit)
}

fn parse_digit(digit: &str) -> Option<u32> {
    if digit.is_empty() {
        return None;
    }

    if digit.starts_with("zero") {
        Some(0)
    } else if digit.starts_with("one") {
        Some(1)
    } else if digit.starts_with("two") {
        Some(2)
    } else if digit.starts_with("three") {
        Some(3)
    } else if digit.starts_with("four") {
        Some(4)
    } else if digit.starts_with("five") {
        Some(5)
    } else if digit.starts_with("six") {
        Some(6)
    } else if digit.starts_with("seven") {
        Some(7)
    } else if digit.starts_with("eight") {
        Some(8)
    } else if digit.starts_with("nine") {
        Some(9)
    } else {
        digit[..1].parse().ok()
    }
}
