fn get_data() -> Vec<Vec<u32>> {
    let input_str = include_str!("./input.txt");
    input_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn part1(terrain: &Vec<Vec<u32>>) -> u32 {
    let mut low_points = Vec::new();

    for y in 0..terrain.len() {
        for x in 0..terrain[y].len() {
            let current = terrain[y][x];

            if (y > 0 && current >= terrain[y - 1][x])
                || (y < terrain.len() - 1 && current >= terrain[y + 1][x])
                || (x > 0 && current >= terrain[y][x - 1])
                || (x < terrain[y].len() - 1 && current >= terrain[y][x + 1])
            {
                continue;
            }

            low_points.push(current + 1);
        }
    }

    low_points.iter().sum()
}

fn part2(terrain: &Vec<Vec<u32>>) -> u32 {
    let mut terrain: Vec<Vec<u32>> = terrain.iter().map(|row| row.clone()).collect();

    let mut to_process = Vec::new();
    let mut basin_sizes = Vec::new();

    for y in 0..terrain.len() {
        for x in 0..terrain[0].len() {
            if terrain[y][x] == 9 {
                continue;
            }

            to_process.push((x, y));
            terrain[y][x] = 9;

            let mut basin_size = 1;

            while to_process.len() > 0 {
                let (x, y) = to_process.pop().unwrap();

                if y > 0 && terrain[y - 1][x] < 9 {
                    to_process.push((x, y - 1));
                    terrain[y - 1][x] = 9;
                    basin_size += 1;
                }

                if y < terrain.len() - 1 && terrain[y + 1][x] < 9 {
                    to_process.push((x, y + 1));
                    terrain[y + 1][x] = 9;
                    basin_size += 1;
                }

                if x > 0 && terrain[y][x - 1] < 9 {
                    to_process.push((x - 1, y));
                    terrain[y][x - 1] = 9;
                    basin_size += 1;
                }

                if x < terrain[y].len() - 1 && terrain[y][x + 1] < 9 {
                    to_process.push((x + 1, y));
                    terrain[y][x + 1] = 9;
                    basin_size += 1;
                }
            }

            basin_sizes.push(basin_size);
        }
    }

    basin_sizes.sort_by(|a, b| b.partial_cmp(a).unwrap());
    basin_sizes[0..3].iter().fold(1, |acc, curr| acc * curr)
}

fn main() {
    let data = get_data();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
