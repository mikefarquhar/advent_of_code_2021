enum Direction {
    Forward,
    Up,
    Down,
}

impl Direction {
    fn from_str(string: &str) -> Option<Self> {
        match string {
            "forward" => Some(Self::Forward),
            "up" => Some(Self::Up),
            "down" => Some(Self::Down),
            _ => None,
        }
    }
}

struct Command {
    direction: Direction,
    distance: i32,
}

fn get_data() -> Vec<Command> {
    let input = include_str!("./input.txt");

    input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_once(' ').unwrap();
            Command {
                direction: Direction::from_str(direction).unwrap(),
                distance: distance.parse().unwrap(),
            }
        })
        .collect()
}

fn part1(commands: &[Command]) {
    let mut position = 0;
    let mut depth = 0;

    for command in commands {
        match command.direction {
            Direction::Forward => position += command.distance,
            Direction::Up => depth -= command.distance,
            Direction::Down => depth += command.distance,
        };
    }

    let multiplied = position * depth;
    println!("Part 1: Multiplied totals: {}", multiplied);
}

fn part2(commands: &[Command]) {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in commands {
        match command.direction {
            Direction::Forward => {
                position += command.distance;
                depth += command.distance * aim;
            },
            Direction::Up => aim -= command.distance,
            Direction::Down => aim += command.distance,
        }
    }

    let multiplied = position * depth;
    println!("Part 2: Multiplied totals: {}", multiplied);
}

fn main() {
    let data = get_data();
    part1(&data);
    part2(&data);
}
