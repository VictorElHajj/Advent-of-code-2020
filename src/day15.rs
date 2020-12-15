use std::collections::HashMap;

fn solver(input: &str, num: usize) -> u32 {
    let mut hs: HashMap<u32, u32> = HashMap::new();

    let mut last_num: u32 = 0;

    for (i, num_str) in input.split(',').enumerate() {
        let num = num_str.parse::<u32>().unwrap();
        hs.insert(num, i as u32 + 1);
        last_num = num;
    }
    hs.remove(&last_num);

    for cur_round in (hs.len() + 1)..num {
        match hs.get_mut(&last_num) {
            Some(prev) => {
                last_num = cur_round as u32 - *prev;
                *prev = cur_round as u32;
            }
            None => {
                hs.insert(last_num, cur_round as u32);
                last_num = 0;
            }
        }
    }
    last_num
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> u32 {
    solver(input, 2020)
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> u32 {
    solver(input, 30000000)
}
