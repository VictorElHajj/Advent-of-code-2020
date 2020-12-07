use std::collections::HashMap;

fn parse_aj(input: &str, hm: &mut HashMap<String, Vec<(String, u8)>>) {
    let (from, to) = input.split_once(" bags contain ").unwrap();
    let to_bags: Vec<(String, u8)> = to
        .split(',')
        .map(|to| {
            if to != "no other bags." {
                let mut iter = to.trim_start().trim_end_matches('.').split(' ');
                let weight = iter.by_ref().next().unwrap().parse::<u8>().unwrap();
                let to = iter.by_ref().take(2).collect::<Vec<&str>>().join(" ");
                (to, weight)
            } else {
                (String::from("none"), 0)
            }
        })
        .collect();
    hm.insert(from.to_string(), to_bags);
}

fn dig(
    key: &String,
    cache: &mut HashMap<String, bool>,
    hm: &HashMap<String, Vec<(String, u8)>>,
) -> bool {
    match cache.get(key) {
        Some(bool) => return *bool,
        None => {
            let mut found = false;
            for (to, _) in hm.get(key).unwrap() {
                match cache.get(to) {
                    Some(true) => return true,
                    Some(false) => continue,
                    None => {
                        if to == "shiny gold" {
                            cache.insert(to.to_string(), true);
                            return true;
                        } else if to == "none" {
                            cache.insert(to.to_string(), false);
                            return false;
                        } else {
                            let test = dig(to, cache, hm);
                            cache.insert(to.to_string(), test);
                            found |= test;
                        }
                    }
                }
            }
            found
        }
    }
}

fn bag_sum(key: &String, hm: &HashMap<String, Vec<(String, u8)>>) -> u32 {
    let mut sum = 1;
    for (to, weight) in hm.get(key).unwrap() {
        if to == "none" {
            return 1;
        }
        sum += *weight as u32 * bag_sum(to, hm);
    }
    sum
}
// Optimization notes, create the hashmap with_capacity, use cache

#[aoc_generator(day7)]
pub fn generator(input: &str) -> HashMap<String, Vec<(String, u8)>> {
    let mut hm = HashMap::with_capacity(1000);
    for line in input.lines() {
        parse_aj(line, &mut hm);
    }
    hm
}

#[aoc(day7, part1)]
pub fn part1(hm: &HashMap<String, Vec<(String, u8)>>) -> u8 {
    let mut total = 0;
    let mut cache: HashMap<String, bool> = HashMap::with_capacity(1000);
    for (k, _) in hm.iter() {
        total += dig(k, &mut cache, hm) as u8;
    }
    total
}

#[aoc(day7, part2)]
pub fn part2(hm: &HashMap<String, Vec<(String, u8)>>) -> u32 {
    bag_sum(&String::from("shiny gold"), hm) - 1
}
