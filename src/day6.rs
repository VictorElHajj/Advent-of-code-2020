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
        let mut already_counted = [false; 26];
        for b in line {
          lines += 1;
          let i = (b - 'a' as u8) as usize;
          if !already_counted[i] {
            cm[i] += 1;
            already_counted[i] = true;
          }
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
        .fold(0, |acc, x| if *x > 0 { acc + 1 } else { acc })
    })
    .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &Vec<FormGroup>) -> usize {
  input
    .iter()
    .map(|hm| {
      let temp = hm
        .char_map
        .iter()
        .fold(0usize, |acc, x| if *x == hm.len { acc + 1 } else { acc });
      temp
    })
    .sum::<usize>()
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
