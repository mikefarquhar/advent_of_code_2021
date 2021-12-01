fn get_numbers() -> Vec<i32> {
    let input: &str = include_str!("input.txt");

    input.split("\r\n")
        .map(|x| { x.parse().unwrap() })
        .collect()
}

fn puzzle1(numbers: &Vec<i32>) {
    let mut deeper_count = 0;
    for i in 0..numbers.len() - 1 {
        let prev_number = numbers[i];
        let number = numbers[i + 1];
        if number > prev_number {
            deeper_count += 1;
        }
    }

    println!("Puzzle 1: Depth increases: {}", deeper_count);
}

fn puzzle2(numbers: &Vec<i32>) {
    let mut deeper_count = 0;
    for i in 0..numbers.len() - 3 {
        let total_a = numbers[i] + numbers[i + 1] + numbers[i + 2];
        let total_b = numbers[i + 1] + numbers[i + 2] + numbers[i + 3];
        if total_b > total_a {
            deeper_count += 1;
        }
    }

    println!("Puzzle 2: Depth increases: {}", deeper_count);
}

fn main() {
    let numbers = get_numbers();
    puzzle1(&numbers);
    puzzle2(&numbers);
}
