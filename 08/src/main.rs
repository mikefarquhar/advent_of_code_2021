struct Panel {
    signals: [u8; 10],
    displays: [u8; 4],
}

fn process_block(block: &str) -> u8 {
    block
        .chars()
        .fold(0, |acc, curr| {
            acc + (1 << (curr as u32 - 'a' as u32))
        })
}

fn get_data() -> Vec<Panel> {
    let input_str = include_str!("./input.txt");
    input_str
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(" | ").unwrap();
            
            let mut signals: [u8; 10] = [0; 10];
            lhs
                .split(' ')
                .enumerate()
                .for_each(|(i, block)| {
                    signals[i] = process_block(block);
                });
            signals.sort_by(|a, b| a.count_ones().cmp(&b.count_ones()));

            let mut displays: [u8; 4] = [0; 4];
            rhs
                .split(' ')
                .enumerate()
                .for_each(|(i, block)| {
                    displays[i] = process_block(block);
                });

            Panel { signals, displays }
        })
        .collect()
}

// Count number of 1s, 4s, 7s & 8s that appear in the output.
fn part1(panels: &[Panel]) -> u32 {
    panels.iter().fold(0, |acc, curr| {
        acc + curr.displays.iter().fold(0, |acc, curr| {
            let num_segments = curr.count_ones();
            acc + (
                num_segments == 2 ||
                num_segments == 3 ||
                num_segments == 4 ||
                num_segments == 7   
            ) as u32
        })
    })
}

// Total of all values on the displays
fn part2(panels: &[Panel]) -> u32 {
    panels.iter().fold(0, |acc, panel| {
        let mut digits = [0; 10];

        // Known digits.
        digits[1] = panel.signals[0];
        digits[7] = panel.signals[1];
        digits[4] = panel.signals[2];
        digits[8] = panel.signals[9];

        // Numbers with 5 segments
        let bd = digits[4] - digits[1];
        for i in 3..=5 {
            match panel.signals[i] {
                signal if (signal & bd) == bd => digits[5] = signal,
                signal if (signal & digits[1]) == digits[1] => digits[3] = signal,
                signal => digits[2] = signal,
            };
        };

        // Numbers with 6 segments
        for i in 6..=8 {
            match panel.signals[i] {
                signal if (signal & digits[3]) == digits[3] => digits[9] = signal,
                signal if (signal & digits[1]) == digits[1] => digits[0] = signal,
                signal => digits[6] = signal,
            }
        }

        let display_num = panel.displays.iter().enumerate()
            .fold(0, |acc, (i, num)| {
                let power = 10_u32.pow(3 - i as u32);
                let digit = digits.iter().position(|digit| digit == num).unwrap();
                acc + power * digit as u32
            });

        acc + display_num
    })
}

fn main() {
    let data = get_data();
    println!("Part 1: Num 1s, 4s, 7s & 8s: {}", part1(&data));
    println!("Part 2: Total of all display No.s: {}", part2(&data));
}
