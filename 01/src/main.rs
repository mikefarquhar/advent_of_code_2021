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
        // Because the windows being compared are offset by 1 index all but the
        // first and last numbers are the same and so can be cancelled out.
        // Additionally casting the bool to an int lets us skip a conditional
        // check.
        count += (numbers[i] < numbers[i + window_size]) as i32;
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
