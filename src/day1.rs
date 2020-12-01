use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[aoc_generator(day1, part1)]
pub fn generator_part1(input: &str) -> BinaryHeap<i32> {
  input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc_generator(day1, part2)]
pub fn generator_part2(input: &str) -> BinaryHeap<Reverse<i32>> {
  input
    .lines()
    .map(|l| l.parse::<i32>().unwrap())
    .map(|l| Reverse(l))
    .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &BinaryHeap<i32>) -> Result<i32, &str> {
  for (i, v1) in input.iter().enumerate() {
    for v2 in input.iter().skip(i) {
      if v1 + v2 == 2020 {
        return Ok(v1 * v2);
      }
    }
  }
  Err("Solution not found")
}

#[aoc(day1, part2)]
pub fn part2(input: &BinaryHeap<Reverse<i32>>) -> Result<i32, &str> {
  for (i, v1) in input.iter().enumerate() {
    for (j, v2) in input.iter().skip(i).enumerate() {
      for v3 in input.iter().skip(j) {
        if v1.0 + v2.0 + v3.0 == 2020 {
          return Ok(v1.0 * v2.0 * v3.0);
        }
      }
    }
  }
  Err("Solution not found")
}
