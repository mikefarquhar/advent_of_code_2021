fn get_data() -> Vec<Vec<u8>> {
    let input_str = include_str!("./input.txt");

    input_str
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn surrounding(x: usize, y: usize, width: usize, height: usize) -> [Option<(usize, usize)>; 8] {
    let above = y > 0;
    let below = y < height - 1;
    let left = x > 0;
    let right = x < width - 1;

    [
        if above && left { Some((x - 1, y - 1)) } else { None },
        if above { Some((x, y - 1)) } else { None },
        if above && right { Some((x + 1, y - 1)) } else { None },
        if left { Some((x - 1, y)) } else { None },
        if right { Some((x + 1, y)) } else { None },
        if below && left { Some((x - 1, y + 1)) } else { None },
        if below { Some((x, y + 1)) } else { None },
        if below && right { Some((x + 1, y + 1)) } else { None },
    ]
}

fn step(
    octos: &Vec<Vec<u8>>,
    next_octos: &mut Vec<Vec<u8>>,
    flash_stack: &mut Vec<(usize, usize)>,
) -> u32 {
    // Phase 1 - Charge
    for y in 0..octos.len() {
        for x in 0..octos[y].len() {
            let energy = octos[y][x] + 1;
            next_octos[y][x] = energy;
            if energy > 9 { flash_stack.push((x, y)); }
        }
    }

    // Phase 2 - Flash
    let mut num_flashes = flash_stack.len() as u32;
    while flash_stack.len() > 0 {
        let (x, y) = flash_stack.pop().unwrap();
        for opt in surrounding(x, y, octos[y].len(), octos.len()) {
            match opt {
                Some((x, y)) => {
                    let energy = next_octos[y][x] + 1;
                    if energy == 10 {
                        num_flashes += 1;
                        flash_stack.push((x, y));
                    }
                    if energy <= 10 {
                        next_octos[y][x] = energy;
                    }
                },
                None => (),
            }
        }
    }

    // Phase 3 - Reset
    for y in 0..next_octos.len() {
        for x in 0..next_octos[y].len() {
            if next_octos[y][x] > 9 {
                next_octos[y][x] = 0;
            }
        }
    }
    
    num_flashes
}

fn part1(octos: &Vec<Vec<u8>>) -> u32 {
    let mut octos: Vec<Vec<u8>> = octos.iter().map(|row| row.clone()).collect();
    let mut next_octos: Vec<Vec<u8>> = octos.iter().map(|row| row.clone()).collect();
    let mut flash_stack = Vec::new();

    (0..100).fold(0, |acc, _| {
        let num_flashes = acc + step(&octos, &mut next_octos, &mut flash_stack);
        std::mem::swap(&mut octos, &mut next_octos);
        num_flashes
    })
}

fn part2(octos: &Vec<Vec<u8>>) -> u32 {
    let mut octos: Vec<Vec<u8>> = octos.iter().map(|row| row.clone()).collect();
    let mut next_octos: Vec<Vec<u8>> = octos.iter().map(|row| row.clone()).collect();
    let mut flash_stack = Vec::new();

    let mut iteration = 0;
    loop {
        let cells_total = octos
            .iter()
            .flatten()
            .map(|cell| *cell as u32)
            .sum::<u32>();

        if cells_total == 0 {
            break;
        }

        step(&octos, &mut &mut next_octos, &mut flash_stack);
        std::mem::swap(&mut octos, &mut next_octos);

        iteration += 1;
    }

    iteration
}

fn main() {
    let octos = get_data();
    println!("Num flashes: {}", part1(&octos));
    println!("Num flashes: {}", part2(&octos));
}