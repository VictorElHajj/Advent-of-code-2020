#[aoc_generator(day1)]
pub fn generator_part1(input: &str) -> Vec<usize> {
  let mut parsed: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();
  parsed.sort();
  parsed
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<usize>) -> Result<usize, &str> {
  for v1 in input.iter() {
    let goal = 2020 - v1;
    let search = input.binary_search(&goal);
    match search {
      Ok(i2) => return Ok(v1 * input[i2]),
      _ => (),
    };
  }
  Err("Solution not found")
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<usize>) -> Result<usize, &str> {
  for (i, v1) in input.iter().enumerate() {
    for v2 in input.iter().skip(i) {
      let goal = 2020 - v1 - v2;
      let search = input.binary_search(&goal);
      match search {
        Ok(i2) => return Ok(v1 * v2 * input[i2]),
        _ => (),
      };
    }
  }
  Err("Solution not found")
}
