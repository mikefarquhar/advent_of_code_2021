use std::collections::HashMap;

#[derive(PartialEq)]
enum CaveType {
    Start,
    End,
    Small,
    Large,
}

struct Cave {
    cave_type: CaveType,
    connects_to: Vec<usize>,
}

impl Cave {
    fn from_str(tag: &str) -> Self {
        let cave_type = match tag {
            "start" => CaveType::Start,
            "end" => CaveType::End,
            tag if tag == tag.to_lowercase() => CaveType::Small,
            _ => CaveType::Large,
        };

        Self {
            cave_type,
            connects_to: Vec::new(),
        }
    }
}

struct CaveSystem {
    caves: Vec<Cave>,
    start: usize,
}

impl CaveSystem {
    fn count_paths(&self, visit_small_twice: bool) -> u32 {
        let mut visits = vec![0; self.caves.len()];
        self.visit_cave(&mut visits, self.start, !visit_small_twice)
    }

    fn visit_cave(&self, visits: &mut Vec<u32>, cave_index: usize, visited_twice: bool) -> u32 {
        visits[cave_index] += 1;
        let num_paths = self.caves[cave_index]
            .connects_to
            .iter()
            .fold(0, |acc, &index| {
                acc + match self.caves[index].cave_type {
                    CaveType::Small if visits[index] < 1 || !visited_twice => {
                        self.visit_cave(visits, index, visits[index] >= 1 || visited_twice)
                    }
                    CaveType::Large => self.visit_cave(visits, index, visited_twice),
                    CaveType::End => 1,
                    _ => 0,
                }
            });

        visits[cave_index] -= 1;
        num_paths
    }
}

fn get_cave_index(
    caves: &mut Vec<Cave>,
    cave_map: &mut HashMap<String, usize>,
    tag: &str,
) -> usize {
    match cave_map.get(tag) {
        Some(&index) => index,
        None => {
            let cave = Cave::from_str(tag);
            let index = caves.len();
            caves.push(cave);
            cave_map.insert(tag.to_owned(), index);
            index
        }
    }
}

fn get_data() -> CaveSystem {
    let input_str = include_str!("./input.txt");

    let mut cave_map = HashMap::new();
    let mut caves = Vec::new();

    for line in input_str.lines() {
        let (from_tag, to_tag) = line.split_once('-').unwrap();

        let from_index = get_cave_index(&mut caves, &mut cave_map, from_tag);
        let to_index = get_cave_index(&mut caves, &mut cave_map, to_tag);

        caves[from_index].connects_to.push(to_index);
        caves[to_index].connects_to.push(from_index);
    }

    let start = caves
        .iter()
        .position(|cave| cave.cave_type == CaveType::Start)
        .unwrap();

    CaveSystem { caves, start }
}

fn main() {
    let cave_system = get_data();
    println!("Part 1: Num paths: {}", cave_system.count_paths(false));
    println!("Part 2: Num paths: {}", cave_system.count_paths(true));
}
