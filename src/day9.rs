use std::collections::BTreeSet;

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse::<u64>().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn part1(numbers: &Vec<u64>) -> Result<u64, &str> {
    let mut last_iter = numbers.iter();
    let mut last = last_iter.by_ref().next().unwrap();
    let mut window: BTreeSet<&u64> = numbers.iter().take(25).collect();
    for next in numbers.iter().skip(25) {
        let mut found = false;
        for num in window.iter() {
            if window.contains(&(next - *num)) {
                found = true;
                break;
            }
        }
        if !found {
            return Ok(*next);
        }
        // Number was okay, insert into window and remove last to continue
        window.remove(last);
        window.insert(next);
        last = last_iter.by_ref().next().unwrap();
    }
    Err("Did not find")
}

#[aoc(day9, part2)]
pub fn part2(numbers: &Vec<u64>) -> u64 {
    let p1 = part1(numbers).unwrap();
    let mut window_size = 3;
    loop {
        for window in numbers.windows(window_size) {
            let sum: u64 = window.iter().sum();
            if sum == p1 {
                return window.iter().min().unwrap() + window.iter().max().unwrap();
            } else if sum > p1 {
                break;
            }
        }
        window_size += 1;
    }
}
