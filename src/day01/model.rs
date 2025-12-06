use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Left(u16),
    Right(u16),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount_str) = s.split_at(1);
        let amount = amount_str
            .parse::<u16>()
            .map_err(|_| "Invalid number".to_string())?;

        match direction {
            "L" => Ok(Command::Left(amount)),
            "R" => Ok(Command::Right(amount)),
            _ => Err(format!("Unknown direction: {}", direction)),
        }
    }
}

#[derive(Debug)]
pub struct Position(u8);

impl Position {
    const SIZE: u8 = 100;

    pub fn new(value: u8) -> Self {
        assert!(value < Self::SIZE, "Position must be between 0 and 99");
        Self(value)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn step(self, command: Command) -> Self {
        let delta: i16 = match command {
            Command::Left(n) => -(n as i16),
            Command::Right(n) => n as i16,
        };

        let current = self.0 as i16;
        let size = Self::SIZE as i16;

        let new_position: i16 = (((current + delta) % size) + size) % size;
        Position(new_position as u8)
    }
}

#[derive(Debug)]
pub struct State {
    position: Position,
    count: u16,
}

impl State {
    pub fn walk(commands: impl IntoIterator<Item = Command>) -> Self {
        let start = State {
            position: Position::new(50),
            count: 0,
        };

        commands.into_iter().fold(start, |state, command| {
            let new_position = state.position.step(command);
            let hit_zero = new_position.value() == 0;

            State {
                position: new_position,
                count: state.count + if hit_zero { 1 } else { 0 },
            }
        })
    }
}
