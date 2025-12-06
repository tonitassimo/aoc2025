use aoc2025::day01::*;

fn main() {
    let commands = include_str!("../../input/day01.txt")
        .lines()
        .map(|line: &str| line.parse::<Command>().unwrap());

    let result = State::walk(commands);

    println!("{:#?}", result);
}
