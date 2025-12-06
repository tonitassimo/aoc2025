use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Left(i32),
    Right(i32),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount_str) = s.split_at(1);
        let amount = amount_str
            .parse::<i32>()
            .map_err(|_| "Invalid number".to_string())?;

        match direction {
            "L" => Ok(Command::Left(amount)),
            "R" => Ok(Command::Right(amount)),
            _ => Err(format!("Unknown direction: {}", direction)),
        }
    }
}
