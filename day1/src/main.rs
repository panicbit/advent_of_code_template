use aoc::aoc;

#[aoc(2023, 1, 1)]
fn main(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| c as u32 - '0' as u32)
                .collect::<Vec<_>>();
            let number = 10 * digits[0] + digits[digits.len() - 1];

            number
        })
        .sum::<u32>()
}
