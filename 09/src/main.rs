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

fn get_basin_minima(terrain: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut minima = Vec::new();

    for y in 0..terrain.len() {
        for x in 0..terrain[y].len() {
            let height = terrain[y][x];

            if (y > 0 && height >= terrain[y - 1][x])
                || (y < terrain.len() - 1 && height >= terrain[y + 1][x])
                || (x > 0 && height >= terrain[y][x - 1])
                || (x < terrain[y].len() - 1 && height >= terrain[y][x + 1])
            {
                continue;
            }

            minima.push((x, y));
        }
    }

    minima
}

fn part1(terrain: &Vec<Vec<u32>>, basin_minima: &Vec<(usize, usize)>) -> u32 {
    basin_minima
        .iter()
        .fold(0, |acc, (x, y)| acc + terrain[*y][*x] + 1)
}

fn part2(terrain: &mut Vec<Vec<u32>>, basin_minima: &Vec<(usize, usize)>) -> u32 {
    let mut floodfill_stack = Vec::new();
    let mut basin_sizes = Vec::new();

    for position in basin_minima {
        let (x, y) = *position;

        if terrain[y][x] == 9 {
            continue;
        }

        floodfill_stack.push((x, y));
        terrain[y][x] = 9;

        let mut basin_size = 1;

        while floodfill_stack.len() > 0 {
            let (x, y) = floodfill_stack.pop().unwrap();

            if y > 0 && terrain[y - 1][x] < 9 {
                floodfill_stack.push((x, y - 1));
                terrain[y - 1][x] = 9;
                basin_size += 1;
            }

            if y < terrain.len() - 1 && terrain[y + 1][x] < 9 {
                floodfill_stack.push((x, y + 1));
                terrain[y + 1][x] = 9;
                basin_size += 1;
            }

            if x > 0 && terrain[y][x - 1] < 9 {
                floodfill_stack.push((x - 1, y));
                terrain[y][x - 1] = 9;
                basin_size += 1;
            }

            if x < terrain[y].len() - 1 && terrain[y][x + 1] < 9 {
                floodfill_stack.push((x + 1, y));
                terrain[y][x + 1] = 9;
                basin_size += 1;
            }
        }

        basin_sizes.push(basin_size);
    }

    basin_sizes.sort_by(|a, b| b.partial_cmp(a).unwrap());
    basin_sizes[0..3].iter().fold(1, |acc, curr| acc * curr)
}

fn main() {
    let mut terrain = get_data();
    let basin_minima = get_basin_minima(&terrain);
    println!("Part 1: {}", part1(&terrain, &basin_minima));
    println!("Part 2: {}", part2(&mut terrain, &basin_minima));
}
