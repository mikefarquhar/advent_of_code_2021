struct Data {
    values: Vec<u32>,
    line_length: usize,
}

fn get_data() -> Data {
    let input_str = include_str!("./input.txt");

    let line_length = input_str.lines().next().unwrap().len();

    let values = input_str
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    Data{ values, line_length }
}

fn most_common_bit(values: &[u32], column: usize) -> u32 {
    let mut total = 0;
    for value in values {
        let flag = 1 << column;
        total += ((value & flag) > 0) as usize
    }
    (total * 2 >= values.len()) as u32
}

fn least_common_bit(values: &[u32], column: usize) -> u32 {
    1 - most_common_bit(values, column)
}

fn filter_values(
    values: Vec<u32>,
    column: usize,
    required_bit: u32
) -> Vec<u32> {
    let flag = 1 << column;
    values
        .into_iter()
        .filter(|value| ((value & flag) > 0) as u32 == required_bit)
        .collect()
}

fn extract_rating(
    data: &Data,
    bit_getter: &dyn Fn(&[u32], usize) -> u32
) -> u32 {
    let mut column = data.line_length as usize - 1;
    let mut values = data.values.clone();
    loop {
        let required_bit = bit_getter(&values, column);
        values = filter_values(values, column, required_bit);
        if values.len() <= 1 || column == 0 { break; }
        column -= 1;
    }
    *values.first().unwrap()
}

fn part1(data: &Data) {    
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for i in 0..data.line_length {
        let flag = 1 << i;
        let bit = most_common_bit(&data.values, i);
        gamma_rate |= flag * bit;
        epsilon_rate |= flag * (1 - bit);
    }

    let multiplied_total = gamma_rate * epsilon_rate;
    println!("part 1: multiplied_total: {}", multiplied_total);
}

fn part2(data: &Data) {
    let oxygen_rating = extract_rating(data, &most_common_bit);
    let scrubber_rating = extract_rating(data, &least_common_bit);
    let multiplied_total = oxygen_rating * scrubber_rating;
    println!("part 2: multiplied total: {:?}", multiplied_total);
}

fn main() {
    let data = get_data();
    part1(&data);
    part2(&data);
}
