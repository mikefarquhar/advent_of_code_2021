use std::collections::HashMap;

struct CoordPair {
    from: (i32, i32),
    to: (i32, i32),
}

impl CoordPair {
    fn iter(&self) -> PointIter {
        PointIter::new(self.from, self.to)
    }
}

struct PointIter {
    next: (i32, i32),
    end: (i32, i32),
    step_x: i32,
    step_y: i32,
}

impl PointIter {
    fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        let step_x = (end.0 - start.0).signum();
        let step_y = (end.1 - start.1).signum();
        PointIter {
            next: start,
            end,
            step_x,
            step_y,
        }
    }
}

impl Iterator for PointIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<<Self>::Item> {
        let curr = self.next;
        self.next = (
            self.next.0 + self.step_x,
            self.next.1 + self.step_y,
        );
        if (self.step_x > 0 && curr.0 > self.end.0) || (self.step_x < 0 && curr.0 < self.end.0) {
            return None;
        }
        if (self.step_y > 0 && curr.1 > self.end.1) || (self.step_y < 0 && curr.1 < self.end.1) {
            return None;
        }
        Some(curr)
    }
}

fn get_data() -> Vec<CoordPair> {
    let input = include_str!("./input.txt");
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let from = left.split_once(',').unwrap();
            let to = right.split_once(',').unwrap();
            CoordPair {
                from: (from.0.parse().unwrap(), from.1.parse().unwrap()),
                to: (to.0.parse().unwrap(), to.1.parse().unwrap()),
            }
        })
        .collect()
}

fn part1(data: &[CoordPair]) {
    let mut intersections = HashMap::new();

    for line in data {
        if line.from.0 == line.to.0 || line.from.1 == line.to.1 {
            for point in line.iter() {
                *intersections.entry(point).or_insert(0) += 1;
            }
        }
    }

    let total_intersections = intersections
        .values()
        .fold(0, |acc, val| acc + (*val > 1) as i32);

    println!("intersections: {}", total_intersections);
}

fn part2(data: &[CoordPair]) {
    let mut intersections = HashMap::new();

    for line in data {
        for point in line.iter() {
            *intersections.entry(point).or_insert(0) += 1;
        }
    }

    let total_intersections = intersections
        .values()
        .fold(0, |acc, val| acc + (*val > 1) as i32);

    println!("intersections: {}", total_intersections);
}

fn main() {
    let data = get_data();
    part1(&data);
    part2(&data);
}
