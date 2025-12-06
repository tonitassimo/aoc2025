use aoc2025::day01;

fn main() {
    let commands = include_str!("../../input/day01.txt")
        .lines()
        .map(|line| line.parse::<day01::Command>().unwrap())
        .collect::<Vec<_>>();

    println!("{:#?}", commands);
}
