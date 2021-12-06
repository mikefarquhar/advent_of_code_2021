struct Lanternfish {
    circle_buffer: [u64; 7],
    pointer: usize,
    new_queue: [u64; 2],
}

impl Lanternfish {
    fn new(circle_buffer: [u64; 7]) -> Self {
        Self {
            circle_buffer,
            pointer: 6,
            new_queue: [0; 2],
        }
    }

    fn tick(&mut self) {
        let to_add = self.new_queue[0];
        self.new_queue[0] = self.new_queue[1];
        self.new_queue[1] = self.circle_buffer[self.pointer];
        self.circle_buffer[self.pointer] += to_add;
        self.pointer = (self.pointer + 1) % self.circle_buffer.len();
    }

    fn count(&self) -> u64 {
        self.circle_buffer.iter().sum::<u64>() + self.new_queue.iter().sum::<u64>()
    }
}

fn get_data() -> Lanternfish {
    let input_str = include_str!("./input.txt");

    let mut circle_buffer = [0; 7];
    input_str.split(',').for_each(|n| {
        let n: usize = n.parse().unwrap();
        circle_buffer[n] += 1;
    });

    Lanternfish::new(circle_buffer)
}

fn lanternfish_after_days(num_days: usize) -> u64 {
    let mut lanternfish = get_data();
    for _ in 0..=num_days {
        lanternfish.tick();
    }
    lanternfish.count()
}

fn part1() {
    let days = 80;
    let num_fish = lanternfish_after_days(days);
    println!("After {} days there are {} lanternfish", days, num_fish);
}

fn part2() {
    let days = 256;
    let num_fish = lanternfish_after_days(days);
    println!("After {} days there are {} lanternfish", days, num_fish);
}

fn main() {
    part1();
    part2();
}
