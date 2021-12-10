struct Submarine {
    x: i32,
    depth: i32,
}

impl Default for Submarine {
    fn default() -> Self {
        Self { x: 0, depth: 0 }
    }
}

impl Submarine {
    pub fn execute(self, cmd: Command) -> Self {
        match cmd {
            Command::Down(d) => Self {
                depth: self.depth + d,
                ..self
            },
            Command::Up(d) => Self {
                depth: self.depth - d,
                ..self
            },
            Command::Forward(d) => Self {
                x: self.x + d,
                ..self
            },
        }
    }
}

struct SubmarineWithAim {
    x: i32,
    depth: i32,
    aim: i32,
}

impl Default for SubmarineWithAim {
    fn default() -> Self {
        Self {
            x: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl SubmarineWithAim {
    pub fn execute(self, cmd: Command) -> Self {
        match cmd {
            Command::Down(d) => Self {
                aim: self.aim + d,
                ..self
            },
            Command::Up(d) => Self {
                aim: self.aim - d,
                ..self
            },
            Command::Forward(d) => Self {
                x: self.x + d,
                depth: self.depth + self.aim * d,
                ..self
            },
        }
    }
}

enum Command {
    Down(i32),
    Up(i32),
    Forward(i32),
}

#[derive(Debug)]
enum Error {
    UnknownCommand(String),
    WrongFormat(Option<String>),
}

fn parse_command(cmd_str: &str) -> Result<Command, Error> {
    let mut l = cmd_str.split(' ');
    let cmd_txt = l.next();

    match cmd_txt {
        Some("down") => {
            let command_input = parse_command_input(l.next())?;
            Ok(Command::Down(command_input))
        }
        Some("up") => {
            let command_input = parse_command_input(l.next())?;
            Ok(Command::Up(command_input))
        }
        Some("forward") => {
            let command_input = parse_command_input(l.next())?;
            Ok(Command::Forward(command_input))
        }
        Some(_) => Err(Error::UnknownCommand(cmd_str.to_string())),
        None => Err(Error::WrongFormat(Some(cmd_str.to_string()))),
    }
}

fn parse_command_input(input_str: Option<&str>) -> Result<i32, Error> {
    match input_str {
        Some(i) => i
            .parse::<i32>()
            .map_err(|_| Error::WrongFormat(input_str.map(String::from))),
        None => Err(Error::WrongFormat(input_str.map(String::from))),
    }
}

pub fn day2_p1(input: &str) -> i32 {
    let sub = input
        .lines()
        .map(|line| parse_command(line).unwrap())
        .fold(Submarine::default(), |sub, cmd| sub.execute(cmd));

    sub.x * sub.depth
}

pub fn day2_p2(input: &str) -> i32 {
    let sub = input
        .lines()
        .map(|line| parse_command(line).unwrap())
        .fold(SubmarineWithAim::default(), |sub, cmd| sub.execute(cmd));

    sub.x * sub.depth
}

#[cfg(test)]
mod tests {
    use super::*;
    // tests here
    #[test]
    fn test_d2_p1() {
        assert_eq!(day2_p1(INPUT), 2322630);
    }

    #[test]
    fn test_d2_p2() {
        assert_eq!(day2_p2(INPUT), 2105273490);
    }

    const INPUT: &str = r#"forward 6
down 2
forward 2
down 8
forward 3
down 6
down 8
down 9
forward 7
forward 8
down 9
down 8
down 9
up 8
forward 1
down 7
down 3
forward 3
forward 1
down 3
forward 3
forward 1
up 8
down 5
down 1
forward 6
forward 2
up 9
down 3
down 8
down 3
down 3
up 2
down 7
down 3
up 5
forward 4
down 9
forward 6
forward 3
forward 1
forward 3
down 2
up 9
down 4
forward 6
down 3
forward 2
down 2
up 5
up 1
forward 3
forward 6
down 6
forward 7
forward 1
down 3
down 8
forward 2
down 7
up 1
up 2
forward 5
down 8
down 8
forward 9
forward 7
forward 2
forward 7
up 6
up 9
down 4
forward 4
forward 4
up 1
down 7
forward 9
forward 3
down 6
down 9
forward 7
forward 4
up 7
up 6
up 8
down 9
forward 1
down 1
forward 8
down 7
forward 5
down 3
down 3
down 8
down 8
down 4
up 4
forward 3
down 8
down 9
up 3
up 8
down 9
up 5
forward 2
forward 5
forward 5
down 8
forward 9
forward 8
down 5
down 9
forward 6
forward 2
forward 3
up 1
forward 1
up 2
up 2
forward 4
forward 8
forward 5
down 1
up 4
forward 5
up 7
down 5
down 5
forward 8
up 2
down 7
down 6
down 5
down 5
down 1
down 8
forward 9
forward 2
up 6
up 4
down 8
forward 1
forward 2
down 2
forward 7
forward 7
forward 3
forward 6
forward 8
down 3
forward 6
up 5
down 3
down 8
up 1
forward 1
down 7
down 3
up 5
forward 6
forward 8
forward 9
up 5
up 5
up 5
forward 8
up 5
down 6
down 7
down 5
up 7
up 1
up 3
forward 8
up 9
down 7
down 4
up 6
up 8
up 9
up 9
forward 5
up 5
forward 2
forward 2
forward 6
up 2
down 8
up 2
forward 5
down 9
up 7
down 9
forward 1
forward 8
up 1
forward 7
forward 2
down 3
forward 3
forward 2
up 9
forward 4
forward 9
down 9
forward 5
forward 1
forward 5
forward 8
up 5
forward 1
down 4
up 8
up 4
up 7
forward 4
down 1
up 6
forward 6
down 2
down 7
forward 4
up 7
forward 7
forward 9
down 5
up 5
forward 4
down 6
forward 1
up 8
up 8
down 8
down 7
forward 7
down 3
forward 7
down 3
down 5
down 4
up 8
down 2
down 2
up 5
forward 9
up 9
forward 2
up 4
forward 4
down 2
down 7
forward 7
down 1
down 6
down 4
forward 6
up 4
forward 4
down 6
down 8
down 3
forward 7
down 3
forward 7
down 7
forward 4
up 9
down 5
forward 7
forward 7
up 6
down 3
forward 9
down 1
forward 4
up 9
down 3
up 9
down 5
up 6
forward 1
forward 9
up 4
down 3
forward 1
down 7
down 2
forward 2
down 6
up 4
down 4
up 9
down 3
down 9
down 4
down 1
up 8
down 2
up 1
forward 5
forward 9
forward 1
up 4
forward 5
down 7
up 6
down 3
forward 8
down 1
down 5
forward 5
down 5
down 7
down 8
down 7
up 6
forward 8
down 8
forward 6
down 6
down 7
down 3
forward 2
down 6
down 8
down 7
down 3
up 1
down 7
forward 8
forward 2
forward 5
down 4
up 4
forward 9
down 9
forward 6
down 7
down 4
down 8
up 9
forward 7
down 4
forward 7
forward 1
forward 7
down 9
down 7
forward 3
forward 3
forward 2
down 5
up 5
forward 5
down 2
forward 7
forward 9
forward 7
down 7
down 9
down 5
forward 2
up 5
down 3
forward 7
down 4
down 3
up 5
down 6
down 3
up 4
forward 3
down 1
forward 6
forward 6
down 8
forward 9
down 2
up 3
down 4
down 5
forward 3
down 9
forward 2
up 3
up 4
forward 9
down 2
forward 9
forward 3
down 4
down 2
down 5
down 4
forward 4
down 1
down 9
down 2
forward 8
down 5
forward 5
up 7
down 5
down 2
forward 5
up 4
down 5
up 3
forward 7
down 9
forward 5
forward 2
forward 1
down 7
down 9
down 2
up 2
up 2
up 4
down 4
down 7
down 3
forward 5
forward 3
up 6
down 6
up 6
up 9
forward 8
forward 4
up 3
forward 1
forward 2
up 5
forward 5
forward 8
forward 7
forward 4
down 1
down 8
down 1
forward 3
up 1
forward 7
forward 4
down 8
forward 7
forward 9
forward 3
down 9
down 9
down 3
up 6
up 1
down 4
forward 5
forward 4
forward 6
forward 8
down 6
down 3
forward 5
forward 6
down 4
down 2
up 3
down 3
down 7
down 5
down 5
forward 6
down 4
forward 1
up 2
forward 3
down 1
down 4
down 9
down 7
down 9
forward 9
down 6
down 3
down 2
down 5
up 8
forward 5
forward 5
forward 4
up 5
forward 1
down 9
down 1
up 5
forward 8
forward 6
forward 5
down 1
up 5
down 8
up 7
down 8
down 2
down 3
forward 2
up 4
down 6
up 6
down 3
down 7
up 3
forward 4
down 3
forward 4
up 9
forward 5
down 2
forward 7
forward 5
up 3
up 2
forward 2
down 8
down 1
down 3
up 5
down 4
forward 4
down 1
forward 9
down 3
down 7
down 4
down 4
forward 7
up 5
forward 4
down 8
up 4
forward 6
down 1
up 4
forward 4
down 6
up 5
up 1
forward 2
down 5
forward 8
forward 6
down 8
down 7
down 7
down 1
forward 5
forward 7
forward 7
forward 7
up 3
forward 9
forward 1
down 9
forward 4
up 8
forward 1
forward 5
forward 4
down 2
forward 4
forward 9
forward 3
down 1
forward 4
forward 9
forward 5
down 5
down 5
forward 7
down 3
forward 4
down 6
forward 7
down 2
down 1
down 5
forward 4
forward 9
down 4
forward 2
down 8
up 5
down 9
forward 8
down 3
up 6
down 2
down 4
forward 4
up 2
down 4
down 4
up 7
down 6
forward 4
down 7
forward 3
down 1
up 1
down 2
down 6
down 4
up 3
down 6
up 2
down 6
forward 3
down 9
forward 5
down 5
down 9
down 9
down 7
forward 9
forward 8
forward 9
up 9
forward 7
forward 4
forward 4
up 5
forward 2
down 1
up 9
forward 2
forward 7
forward 1
down 9
forward 9
up 8
up 1
up 7
up 7
down 5
forward 2
forward 8
forward 6
down 7
forward 1
down 9
down 4
down 4
down 1
up 7
forward 4
forward 6
up 5
forward 2
down 9
down 7
forward 1
forward 2
down 5
forward 3
forward 8
forward 6
forward 3
forward 2
down 1
forward 1
forward 1
forward 3
down 9
up 9
down 9
down 6
forward 7
down 6
forward 9
down 9
down 7
down 1
down 9
up 9
down 6
forward 9
down 6
forward 3
down 8
up 5
forward 5
forward 8
up 3
down 8
up 6
forward 4
down 2
forward 6
down 9
forward 6
forward 4
forward 9
forward 3
down 2
down 4
forward 5
down 9
up 7
forward 4
up 1
forward 1
down 6
forward 3
forward 7
forward 2
forward 2
down 5
down 9
down 3
down 5
up 3
forward 1
down 2
down 4
down 1
up 9
up 5
up 1
down 1
up 9
down 5
up 3
up 3
down 7
forward 4
down 6
forward 2
forward 7
forward 4
down 2
forward 6
forward 2
down 3
up 3
up 9
forward 9
forward 9
forward 6
down 8
down 1
forward 9
up 1
down 6
forward 6
up 5
forward 2
forward 6
down 9
forward 1
forward 8
down 8
forward 4
forward 7
up 6
up 1
forward 7
forward 3
forward 2
down 4
down 7
down 7
down 1
down 6
forward 1
down 9
up 9
up 9
down 2
down 2
forward 5
up 2
forward 7
up 5
down 9
forward 7
forward 2
down 8
up 1
down 5
forward 6
down 8
down 7
forward 4
up 2
down 8
forward 2
down 5
down 4
down 9
down 1
down 9
down 6
down 3
forward 1
forward 6
up 1
up 1
up 9
down 2
down 2
forward 5
down 3
forward 4
down 3
down 7
down 7
forward 4
up 3
forward 4
down 3
forward 8
forward 1
up 2
up 1
forward 1
down 6
down 1
down 3
forward 7
down 7
forward 4
forward 5
forward 3
down 5
forward 9
forward 5
down 7
forward 6
down 4
down 4
down 9
down 3
up 9
forward 7
down 7
forward 6
down 2
down 9
forward 4
forward 1
forward 4
down 5
forward 7
down 9
down 8
forward 9
forward 1
down 9
forward 6
up 5
forward 9
down 1
down 5
forward 4
forward 5
forward 8
down 5
forward 9
down 6
down 2
up 4
up 8
forward 3
forward 4
down 3
forward 4
up 6
forward 3
forward 8
forward 7
down 1
down 9
down 8
down 8
down 1
forward 9
up 4
down 5
forward 7
down 8
down 3
forward 9
down 5
forward 7
forward 2
down 4
forward 2
forward 7
down 6
forward 7
down 2
forward 9
down 9
forward 8
forward 8
down 6
forward 7
down 8
forward 7
forward 3
down 1
up 8
down 5
down 6
up 5
forward 5
forward 5
up 5
up 3
up 7
down 6
forward 8
forward 4
down 2
up 5
forward 8
down 6
forward 4
forward 2
up 8
down 8
down 5
down 4
forward 9
forward 9
forward 6
forward 6
down 3
up 1
down 4
down 8
down 9
down 1
forward 3
forward 1
down 9
down 3
down 7
forward 6
forward 9
down 8
down 8
forward 6
forward 1
down 3
forward 1
down 8
down 3
down 9
up 1
forward 6
up 2
down 3
forward 4
forward 2
up 2
down 5
forward 1
down 3
forward 9
forward 4
forward 6
down 3
forward 7
down 6
up 3
up 7
up 5
down 4
forward 4
up 1
forward 7
up 9
forward 3
up 1
down 3
down 4
forward 4
up 3
down 6
down 9
down 6
forward 4
down 9
down 6
forward 4
forward 3
down 3
up 7
down 9
forward 8"#;
}
