fn get_numbers() -> Vec<i32> {
    let input: &str = include_str!("input.txt");

    input.split("\r\n")
        .map(|x| { x.parse().unwrap() })
        .collect()
}

/// Counts the windows that increase in total value when compared to the
/// previous window.
/// 
/// Windows are contiguous, fixed size slices of the array that overlap starting
/// at the beginning and increasing in their starting index by 1 until no more
/// full windows can be created.
fn count_increases_windowed(numbers: &[i32], window_size: usize) -> i32 {
    let mut count = 0;
    for i in 0..numbers.len() - window_size {
        let total_a: i32 = numbers[i..(i + window_size)].iter().sum();
        let total_b: i32 = numbers[(i + 1)..=(i + window_size)].iter().sum();
        if total_b > total_a {
            count += 1;
        }
    }
    count
}

fn puzzle1(numbers: &[i32]) {
    let deeper_count = count_increases_windowed(numbers, 1);
    println!("Puzzle 1: Depth increases: {}", deeper_count);
}

fn puzzle2(numbers: &[i32]) {
    let deeper_count = count_increases_windowed(numbers, 3);
    println!("Puzzle 2: Depth increases: {}", deeper_count);
}

fn main() {
    let numbers = get_numbers();
    puzzle1(&numbers);
    puzzle2(&numbers);
}
