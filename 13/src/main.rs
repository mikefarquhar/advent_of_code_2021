use std::fmt::Display;

struct Sheet {
    dots: Vec<bool>,
    width: usize,
    height: usize,
    original_width: usize,
}

impl Sheet {
    fn from_points(width: usize, height: usize, points: Vec<(usize, usize)>) -> Self {
        let mut dots = vec![false; width * height];

        points.iter().for_each(|(x, y)| dots[y * width + x] = true);

        Self {
            dots,
            width,
            height,
            original_width: width,
        }
    }

    fn fold_at(&mut self, fold: &Fold) {
        match fold {
            &Fold::X(fold_index) => {
                for i in 0..(self.width - fold_index - 1) {
                    for y in 0..self.height {
                        let index_left = self.index(fold_index - i - 1, y);
                        let index_right = self.index(fold_index + i + 1, y);
                        self.dots[index_left] = self.dots[index_left] || self.dots[index_right];
                    }
                }
                self.width = fold_index;
            }
            &Fold::Y(fold_index) => {
                for x in 0..self.width {
                    for i in 0..(self.height - fold_index - 1) {
                        let index_above = self.index(x, fold_index - i - 1);
                        let index_below = self.index(x, fold_index + i + 1);
                        self.dots[index_above] = self.dots[index_above] || self.dots[index_below];
                    }
                }
                self.height = fold_index;
            }
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.original_width + x
    }

    fn num_dots(&self) -> u32 {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| self.dots[self.index(x, y)]))
            .filter(|&is_dot| is_dot)
            .count() as u32
    }
}

impl Display for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            let start = y * self.original_width;
            let end = y * self.original_width + self.width;
            let line: String = self.dots[start..end]
                .iter()
                .map(|&is_dot| if is_dot { '█' } else { ' ' })
                .collect();
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

enum Fold {
    X(usize),
    Y(usize),
}

fn get_data() -> (Sheet, Vec<Fold>) {
    let input_str = include_str!("./input.txt");
    let (points_str, folds_str) = input_str.split_once("\n\n").unwrap();

    let mut width = 0;
    let mut height = 0;

    let points: Vec<(usize, usize)> = points_str
        .lines()
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').unwrap();
            let x = x_str.parse::<usize>().unwrap();
            let y = y_str.parse::<usize>().unwrap();
            width = width.max(x + 1);
            height = height.max(y + 1);
            (x, y)
        })
        .collect();

    let sheet = Sheet::from_points(width, height, points);

    let folds = folds_str
        .lines()
        .map(|line| {
            let fold_str = &line[11..];
            let (axis_str, index_str) = fold_str.split_once('=').unwrap();
            let index = index_str.parse::<usize>().unwrap();
            match axis_str {
                "x" => Fold::X(index),
                "y" => Fold::Y(index),
                _ => unreachable!(),
            }
        })
        .collect();

    (sheet, folds)
}

fn main() {
    let (mut sheet, folds) = get_data();
    let mut folds_iter = folds.iter();

    // Part 1:
    let first = folds_iter.next().unwrap();
    sheet.fold_at(first);
    println!("Part 1: {} dots after first fold", sheet.num_dots());

    // Part 2:
    folds_iter.for_each(|fold| sheet.fold_at(fold));
    println!("Part 2: Activation code");
    println!("{}", sheet);
}
