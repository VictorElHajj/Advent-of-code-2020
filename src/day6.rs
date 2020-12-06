pub struct FormGroup {
  char_map: [u8; 26],
  len: u8,
}

#[aoc_generator(day6)]
pub fn generator(input: &str) -> Vec<FormGroup> {
  input
    .split("\n\n")
    .map(|group| {
      let mut cm = [0u8; 26];
      let mut lines = 0;
      for line in group.as_bytes().split(|b| *b as char == '\n') {
        lines += 1;
        for b in line {
          let mut already_counted = [false; 26];
          let i = (b - 'a' as u8) as usize;
          cm[i] += !already_counted[i] as u8;
          already_counted[i] = false;
        }
      }
      FormGroup {
        char_map: cm,
        len: lines,
      }
    })
    .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &Vec<FormGroup>) -> usize {
  input
    .iter()
    .map(|hm| {
      hm.char_map
        .iter()
        .fold(0, |acc, x| acc + (*x > 0 as u8) as usize)
    })
    .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &Vec<FormGroup>) -> usize {
  input
    .iter()
    .map(|hm| {
      hm.char_map
        .iter()
        .fold(0, |acc, x| acc + (*x == hm.len as u8) as usize)
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test() {
    let test_input = "\
abc

a
b
c

ab
ac

a
a
a
a

b";
    let parsed = generator(&test_input);
    //let result1 = part1(&parsed);
    let result2 = part2(&parsed);
    //assert_eq!(result1, 11);
    assert_eq!(result2, 6)
  }
}
