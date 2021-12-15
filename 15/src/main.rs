use std::{cmp::Ordering, collections::BinaryHeap};

struct Grid {
    cells: Vec<u32>,
    width: usize,
    height: usize,
}

impl Grid {
    fn adjacent_indices(&self, index: usize) -> [Option<usize>; 4] {
        [
            if index % self.width > 0 {
                Some(index - 1)
            } else {
                None
            },
            if index % self.width < self.width - 1 {
                Some(index + 1)
            } else {
                None
            },
            if index > self.width {
                Some(index - self.width)
            } else {
                None
            },
            if index < self.cells.len() - self.width {
                Some(index + self.width)
            } else {
                None
            },
        ]
    }

    fn dist_manhattan(&self, start: usize, end: usize) -> u32 {
        let sx = (start % self.width) as i32;
        let sy = (start / self.width) as i32;

        let ex = (end % self.width) as i32;
        let ey = (end / self.width) as i32;

        ((ex - sx).abs() + (ey - sy).abs()) as u32
    }

    fn shortest_path(&self) -> u32 {
        let start_index = 0;
        let end_index = self.cells.len() - 1;

        let mut costs: Vec<_> = (0..self.cells.len()).map(|_| u32::MAX).collect();
        costs[start_index] = 0;

        let mut heap = BinaryHeap::new();
        heap.push(SearchNode {
            index: start_index,
            priority: 0,
        });

        while let Some(SearchNode { index, priority: _ }) = heap.pop() {
            let cost = costs[index];
            if index == end_index {
                return cost;
            }

            for adjacent_index in self.adjacent_indices(index).iter().filter_map(|&i| i) {
                let next_cost = cost + self.cells[adjacent_index];
                if next_cost < costs[adjacent_index] {
                    costs[adjacent_index] = next_cost;

                    let priority = next_cost + self.dist_manhattan(adjacent_index, end_index);
                    heap.push(SearchNode {
                        index: adjacent_index,
                        priority,
                    });
                }
            }
        }

        unreachable!()
    }

    fn expand_times(&mut self, scale_x: usize, scale_y: usize) {
        let new_width = self.width * scale_x;
        let new_height = self.height * scale_y;

        let mut new_cells = vec![0; new_width * new_height];

        for tile_y in 0..scale_y {
            for original_y in 0..self.height {
                let new_y = tile_y * self.height + original_y;

                for tile_x in 0..scale_x {
                    for original_x in 0..self.width {
                        let new_x = tile_x * self.width + original_x;

                        let original_index = original_y * self.width + original_x;
                        let new_index = new_y * new_width + new_x;

                        let risk_boost = (tile_y + tile_x) as i32;
                        let current_value = self.cells[original_index] as i32;
                        let value = ((current_value + risk_boost - 1) % 9 + 1) as u32;

                        new_cells[new_index] = value;
                    }
                }
            }
        }

        self.cells = new_cells;
        self.width = new_width;
        self.height = new_height;
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchNode {
    index: usize,
    priority: u32,
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_data() -> Grid {
    let input_str = include_str!("./input.txt");
    let height = input_str.lines().count();

    let cells: Vec<_> = input_str
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|digit| char::to_digit(digit, 10).unwrap() as u32)
        })
        .collect();

    let width = cells.len() / height;

    Grid {
        cells,
        width,
        height,
    }
}

fn main() {
    let mut grid = get_data();
    println!("Part 1: Shortest path: {}", grid.shortest_path());
    grid.expand_times(5, 5);
    println!("Part 2: Shortest path: {}", grid.shortest_path());
}
