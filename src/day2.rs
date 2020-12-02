use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct PasswordPolicy {
  min: u8,
  max: u8,
  character: char,
  password: String,
}

impl FromStr for PasswordPolicy {
  type Err = std::num::ParseIntError;
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let mut iterator = input.chars();
    let min: u8 = iterator
      .by_ref()
      .take_while(|c| *c != '-')
      .collect::<String>()
      .parse::<u8>()
      .unwrap();
    let max: u8 = iterator
      .by_ref()
      .take_while(|c| *c != ' ')
      .collect::<String>()
      .parse::<u8>()
      .unwrap();
    let character: char = iterator.by_ref().next().unwrap();
    let password: String = iterator.skip(2).collect::<String>();
    Ok(PasswordPolicy {
      min,
      max,
      character,
      password,
    })
  }
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<PasswordPolicy> {
  input
    .lines()
    .map(|l| l.parse::<PasswordPolicy>().unwrap())
    .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<PasswordPolicy>) -> usize {
  input
    .iter()
    .filter(|pp| {
      let matches: u8 = pp.password.matches(pp.character).count() as u8;
      matches >= pp.min && matches <= pp.max
    })
    .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<PasswordPolicy>) -> usize {
  input
    .iter()
    .filter(|pp| {
      let first = pp.password.as_bytes()[(pp.min - 1) as usize] as char == pp.character;
      let second = pp.password.as_bytes()[(pp.max - 1) as usize] as char == pp.character;
      first ^ second
    })
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  // Parsing password policy, modified from example by making some of the min and maxs 2 characters
  fn test_generator() {
    let test_input = "1-30 a: abcde
1-3 b: cdefg
10-11 c: ccccccccc";
    let parsed: Vec<PasswordPolicy> = generator(test_input);
    let expected_result = vec![
      PasswordPolicy {
        min: 1,
        max: 30,
        character: 'a',
        password: String::from("abcde"),
      },
      PasswordPolicy {
        min: 1,
        max: 3,
        character: 'b',
        password: String::from("cdefg"),
      },
      PasswordPolicy {
        min: 10,
        max: 11,
        character: 'c',
        password: String::from("ccccccccc"),
      },
    ];
    assert_eq!(parsed, expected_result)
  }
}
