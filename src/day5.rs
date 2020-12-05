#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<u16> {
  let mut rows: Vec<u16> = input
    .as_bytes()
    .chunks(11)
    .map(|line| {
      let col = col(line);
      let row = row(line);
      col + row * 8
    })
    .collect();
  rows.sort_unstable();
  rows
}

#[aoc(day5, part1)]
pub fn part_1(input: &Vec<u16>) -> u16 {
  *input.iter().last().unwrap()
}

#[aoc(day5, part2)]
pub fn part_2(input: &Vec<u16>) -> Result<u16, &str> {
  let mut prev_seat_id = 0;
  for id in input {
    if *id == prev_seat_id + 2 && prev_seat_id != 0 {
      return Ok(id - 1);
    }
    prev_seat_id = *id;
  }
  Err("Could not find seat id")
}

pub fn row(input: &[u8]) -> u16 {
  let row = input.iter().take(7);
  let mut result = 0;
  for (i, b) in row.enumerate() {
    if *b as char == 'B' {
      result |= 1 << (6 - i);
    }
  }
  result
}

pub fn col(input: &[u8]) -> u16 {
  let row = input.iter().skip(7).take(3);
  let mut result = 0;
  for (i, b) in row.enumerate() {
    if *b as char == 'R' {
      result |= 1 << (2 - i);
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_generator() {
    let test_input = "\
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";
    let parsed = generator(&test_input);
    let expexted_result = [119, 567, 820];
    assert_eq!(parsed, expexted_result)
  }
}
