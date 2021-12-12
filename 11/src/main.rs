use std::{collections::VecDeque};

#[derive(Clone)]
struct OctoGrid {
    cells: Vec<u8>,
    flash_queue: VecDeque<usize>,
    width: usize,
    height: usize,
}

impl OctoGrid {
    fn new(cells: Vec<u8>, width: usize) -> Self {
        let height = cells.len() / width;
        let flash_queue = VecDeque::new();
        Self { cells, flash_queue, width, height }
    }

    fn kernel(&self, i: usize) -> impl Iterator<Item = usize> {
        let y = i / self.width;
        let x = i % self.width;

        let min_x = if x > 0 { x - 1 } else { x };
        let max_x = if x < self.width - 1 { x + 1 } else { x };
        let min_y = if y > 0 { y - 1 } else { y };
        let max_y = if y < self.height - 1 { y + 1 } else { y };

        let width = self.width;
        (min_y..=max_y).flat_map(move |y| {
            (min_x..=max_x).map(move |x| y * width + x)
        })
    }

    fn step(&mut self) -> u32 {
        // Phase 1 - Charge
        for i in 0..self.cells.len() {
            let energy = self.cells[i];
            if energy == 9 {
                self.flash_queue.push_back(i);
            }
            self.cells[i] = (energy + 1) % 10;
        }

        // Phase 2 - Flash
        let mut num_flashes = self.flash_queue.len() as u32;
        while self.flash_queue.len() > 0 {
            let center = self.flash_queue.pop_front().unwrap();
            for i in self.kernel(center) {
                let energy = self.cells[i];
                if energy == 9 {
                    num_flashes += 1;
                    self.flash_queue.push_back(i);
                    self.cells[i] = 0;
                }
                else if energy > 0 {
                    self.cells[i] = energy + 1;
                }
            }
        }

        num_flashes
    }

    fn all_flashed(&self) -> bool {
        self.cells.iter().fold(0, |acc, &curr| acc + curr as u32) == 0
    }
}

fn get_data() -> OctoGrid {
    let input_str = include_str!("./input.txt");

    let cells: Vec<u8> = input_str
        .lines()
        .flat_map(|line| line.chars().map(|char| char.to_digit(10).unwrap() as u8))
        .collect();

    let width = input_str.lines().next().unwrap().len();

    OctoGrid::new(cells, width)
}

fn part1(octos: &OctoGrid) -> u32 {
    let mut octos = octos.clone();
    (0..100).fold(0, |acc, _| acc + octos.step())
}

fn part2(octos: &OctoGrid) -> u32 {
    let mut octos = octos.clone();
    let mut iterations = 0;
    while !octos.all_flashed() {
        octos.step();
        iterations += 1;
    }
    iterations
}

fn main() {
    let octos = get_data();
    println!("Num flashes: {}", part1(&octos));
    println!("Num iterations for sync: {}", part2(&octos));
}
