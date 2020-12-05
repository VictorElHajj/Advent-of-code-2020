#[derive(Debug, PartialEq)]
pub struct Seat {
  row: u16,
  col: u16,
  id: u16,
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<Seat> {
  let mut rows: Vec<Seat> = input
    .as_bytes()
    .split(|b| *b as char == '\n')
    .map(|line| {
      let col = col(line);
      let row = row(line);
      Seat {
        row,
        col,
        id: col + row * 8,
      }
    })
    .collect();
  rows.sort_unstable_by(|x, y| x.id.cmp(&y.id));
  rows
}

#[aoc(day5, part1)]
pub fn part_1(input: &Vec<Seat>) -> u16 {
  input.iter().last().unwrap().id
}

#[aoc(day5, part2)]
pub fn part_2(input: &Vec<Seat>) -> Result<u16, &str> {
  let mut prev_seat_id = 0;
  for seat in input {
    if seat.id == prev_seat_id + 2 && prev_seat_id != 0 {
      return Ok(seat.id - 1);
    }
    prev_seat_id = seat.id;
  }
  Err("Could not find seat id")
}

pub fn row(input: &[u8]) -> u16 {
  let row = input.iter().take(7);
  let mut min = 0;
  let mut max = 127;
  for b in row {
    let mid = (min + max) / 2;
    if *b as char == 'B' {
      min = mid;
    } else if *b as char == 'F' {
      max = mid;
    } else {
      unreachable!();
    }
  }
  max
}

pub fn col(input: &[u8]) -> u16 {
  let row = input.iter().skip(7).take(3);
  let mut min = 0;
  let mut max = 7;
  for b in row {
    let mid = (min + max) / 2;
    if *b as char == 'R' {
      min = mid;
    } else if *b as char == 'L' {
      max = mid;
    } else {
      unreachable!();
    }
  }
  max
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
    let expexted_result = [
      Seat {
        row: 70,
        col: 7,
        id: 567,
      },
      Seat {
        row: 14,
        col: 7,
        id: 119,
      },
      Seat {
        row: 102,
        col: 4,
        id: 820,
      },
    ];
    assert_eq!(parsed, expexted_result)
  }
}
