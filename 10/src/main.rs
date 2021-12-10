fn get_data() -> Vec<&'static str> {
    let data = include_str!("./input.txt");
    data.lines().collect()
}

fn score_lines(lines: &Vec<&str>) -> (u64, u64) {
    let mut error_score = 0;
    let mut autocomplete_scores = Vec::new();
    let mut brace_stack = Vec::new();

    for line in lines {
        let start_error = error_score;

        for char in line.chars() {
            match char {
                '(' => brace_stack.push(')'),
                '[' => brace_stack.push(']'),
                '{' => brace_stack.push('}'),
                '<' => brace_stack.push('>'),
                closing_brace => {
                    if Some(closing_brace) != brace_stack.pop() {
                        error_score += match closing_brace {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => unreachable!(),
                        };
                        break;
                    }
                }
            };
        }

        if error_score == start_error {
            let autocomplete_score = brace_stack.iter().rev().fold(0, |acc, char| {
                acc * 5
                    + match char {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            });
            autocomplete_scores.push(autocomplete_score);
        }

        brace_stack.clear();
    }

    autocomplete_scores.sort();
    let autocomplete_score = autocomplete_scores[autocomplete_scores.len() / 2];

    (error_score, autocomplete_score)
}

fn main() {
    let lines = get_data();
    let (error_score, autocomplete_score) = score_lines(&lines);
    println!("Syntax error score: {}", error_score);
    println!("Autocomplete score: {}", autocomplete_score);
}
