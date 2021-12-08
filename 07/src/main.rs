fn get_data() -> Vec<i32> {
    let input_str = include_str!("./input.txt");
    let mut data: Vec<i32> = input_str
        .split(',')
        .map(|num_str| num_str.parse().unwrap())
        .collect();
    data.sort();
    data
}

// Writing out the equation f(b) = sumᵢ₌₁₋ₙ(|b - aᵢ|) and differentiating shows
// that the optimal solution must lie at the median of the starting points.
fn part1(crabs: &[i32]) -> i32 {
    let index = crabs.len() / 2;
    let offset = 1 - crabs.len() % 2;
    let median = (crabs[index - offset] + crabs[index]) / 2;

    crabs.iter().fold(0, |total_fuel_cost, curr_pos| {
        total_fuel_cost + (median - curr_pos).abs()
    })
}

// Similarly to the above, differentiating shows the optimal solution lies at
// the arithmetic mean + some amount in the range -0.5..0.5. As we're dealing
// with integers we can use the fractional part of the mean to tell if we need
// to check (mean-1)..(mean) or (mean)..(mean+1).
fn part2(crabs: &[i32]) -> i32 {
    let mean = crabs.iter().sum::<i32>() as f32 / crabs.len() as f32;
    let offset = (mean.fract() < 0.5) as i32;
    let start = mean as i32 - offset;

    ((start)..=(start + 1))
        .map(|end_pos| {
            crabs.iter().fold(0, |total_fuel_cost, curr_pos| {
                total_fuel_cost + ((end_pos - curr_pos).pow(2) + (end_pos - curr_pos).abs()) / 2
            })
        })
        .min()
        .unwrap()
}

fn main() {
    let crabs = get_data();
    println!("Part 1: min fuel cost: {}", part1(&crabs));
    println!("Part 2: min fuel cost: {}", part2(&crabs));
}
