use std::collections::HashMap;

fn get_data() -> (Vec<char>, HashMap<(char, char), char>) {
    let input_str = include_str!("./input.txt");
    let (template_str, rules_str) = input_str.split_once("\n\n").unwrap();

    let template: Vec<char> = template_str.chars().collect();

    let mut rules = HashMap::new();
    rules_str
        .lines()
        .for_each(|line| {
            let (matcher_str, insert_str) = line.split_once(" -> ").unwrap();
            let match_0 = matcher_str.chars().nth(0).unwrap();
            let match_1 = matcher_str.chars().nth(1).unwrap();
            let insert = insert_str.chars().nth(0).unwrap();
            rules.insert((match_0, match_1), insert);
        });

    (template, rules)
}

fn run_iterations(iterations: u32, template: &Vec<char>, rules: &HashMap<(char, char), char>) -> u64 {
    let mut single_counts = HashMap::new();
    let mut pair_counts = HashMap::new();
    let mut pair_counts_prev = HashMap::new();

    single_counts.insert(template[0], 1);
    for i in 0..template.len() - 1 {
        *single_counts.entry(template[i + 1]).or_insert(0) += 1;
        let pair = (template[i], template[i + 1]);
        *pair_counts.entry(pair).or_insert(0) += 1;
    }

    for _ in 0..iterations {
        std::mem::swap(&mut pair_counts, &mut pair_counts_prev);
        pair_counts.clear();

        for (prev_pair, count) in &pair_counts_prev {
            let &new_char = rules.get(prev_pair).unwrap();
            *single_counts.entry(new_char).or_insert(0) += count;

            let pair_left = (prev_pair.0, new_char);
            let pair_right = (new_char, prev_pair.1);

            *pair_counts.entry(pair_left).or_insert(0) += count;
            *pair_counts.entry(pair_right).or_insert(0) += count;
        }
    }

    let mut min_count = u64::MAX;
    let mut max_count = u64::MIN;
    for &count in single_counts.values() {
        min_count = min_count.min(count);
        max_count = max_count.max(count);
    }

    max_count - min_count
}

fn main() {
    let (template, rules) = get_data();
    println!("10 iterations: {}", run_iterations(10, &template, &rules));
    println!("40 iterations: {}", run_iterations(40, &template, &rules));
}