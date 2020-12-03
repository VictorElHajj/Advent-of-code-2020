pub enum Tile {
  Empty,
  Tree,
}

#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<Vec<Tile>> {
  input
    .as_bytes()
    .split(|b| *b as char == '\n')
    .map(|r| {
      r.iter()
        .map(|b| match *b as char {
          '.' => Tile::Empty,
          '#' => Tile::Tree,
          _ => unreachable!(),
        })
        .collect::<Vec<Tile>>()
    })
    .collect::<Vec<Vec<Tile>>>()
}

pub fn toboggan(input: &Vec<Vec<Tile>>, slope: &(usize, usize)) -> u32 {
  let mut trees = 0;
  for (height, row) in input.iter().enumerate().step_by(slope.1) {
    trees += match row.iter().cycle().nth(slope.0 * height / slope.1).unwrap() {
      Tile::Empty => 0,
      Tile::Tree => 1,
    }
  }
  trees
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<Tile>>) -> u32 {
  let slope = (3, 1);
  toboggan(input, &slope)
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Vec<Tile>>) -> u32 {
  let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
  slopes
    .iter()
    .map(|slope| toboggan(input, slope))
    .fold(1, |tot, trees| tot * trees)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_part2() {
    let test_input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    let parsed = generator(&test_input);
    let result = part2(&parsed);
    assert_eq!(result, 2 * 7 * 3 * 4 * 2)
  }
}
