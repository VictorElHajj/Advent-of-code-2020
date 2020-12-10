use itertools::Itertools;

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<u32> {
    let mut sorted_output = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    sorted_output.push(0);
    sorted_output.sort();
    sorted_output.push(sorted_output.last().unwrap() + 3);
    sorted_output
}

#[aoc(day10, part1)]
pub fn part1(numbers: &Vec<u32>) -> u32 {
    let mut ones = 0;
    let mut trees = 0;
    for (x, y) in numbers.iter().tuple_windows() {
        if y - x == 1 {
            ones += 1;
        } else if y - x == 3 {
            trees += 1;
        }
    }
    ones * trees
}

fn tribonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        2 => 2,
        n => tribonacci(n - 1) + tribonacci(n - 2) + tribonacci(n - 3),
    }
}

#[aoc(day10, part2)]
pub fn part2(numbers: &Vec<u32>) -> u64 {
    let mut run = false;
    let mut run_length = 0;
    let mut tot: u64 = 1;
    for (x, y) in numbers.iter().tuple_windows() {
        if y - x == 1 {
            run_length += 1;
            if !run {
                run = true;
            }
        } else {
            if run {
                tot *= tribonacci(run_length as u64);
                run_length = 0;
                run = false;
            }
        }
    }
    tot
}
