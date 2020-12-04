use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<HashMap<String, String>> {
  input
    .split("\n\n")
    .collect::<Vec<&str>>()
    .par_iter()
    .map(|raw_passport| {
      let segments: Vec<&str> = raw_passport.split(&[':', ' ', '\n'][..]).collect();
      segments
        .iter()
        .tuples()
        .map(|(a, b)| (a.to_owned().to_owned(), b.to_owned().to_owned()))
        .collect()
    })
    .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<HashMap<String, String>>) -> usize {
  input
    .par_iter()
    .filter(|hm| hm.len() - (hm.contains_key("cid") as usize) == 7)
    .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<HashMap<String, String>>) -> usize {
  input
    .par_iter()
    .filter(|hm| {
      let has_all_fields = hm.len() - (hm.contains_key("cid") as usize) == 7;
      if !has_all_fields {
        return false;
      }
      let byr = hm.get("byr").unwrap().parse::<u32>().unwrap();
      let valid_byr = hm.get("byr").unwrap().len() == 4 && byr >= 1920 && byr <= 2002;
      let iyr = hm.get("iyr").unwrap().parse::<u32>().unwrap();
      let valid_iyr = hm.get("iyr").unwrap().len() == 4 && iyr >= 2010 && iyr <= 2020;
      let eyr = hm.get("eyr").unwrap().parse::<u32>().unwrap();
      let valid_eyr = hm.get("eyr").unwrap().len() == 4 && eyr >= 2020 && eyr <= 2030;
      let hcl = hm.get("hcl").unwrap();
      let valid_hcl;
      if hcl.chars().next().unwrap() == '#' {
        valid_hcl = true;
      } else {
        return false;
      }
      let hgt = hm.get("hgt").unwrap();
      let valid_hgt;
      if hgt.contains("cm") {
        let hgt_value = hgt
          .chars()
          .take(hgt.len() - 2)
          .collect::<String>()
          .parse::<u32>()
          .unwrap();
        valid_hgt = hgt_value >= 150 && hgt_value <= 193;
      } else if hgt.contains("in") {
        let hgt_value = hgt
          .chars()
          .take(hgt.len() - 2)
          .collect::<String>()
          .parse::<u32>()
          .unwrap();
        valid_hgt = hgt_value >= 59 && hgt_value <= 76;
      } else {
        return false;
      }
      let ecl = hm.get("ecl").unwrap();
      let valid_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .any(|color| color == ecl);
      let valid_pid = hm.get("pid").unwrap().len() == 9;
      has_all_fields
        && valid_byr
        && valid_iyr
        && valid_ecl
        && valid_pid
        && valid_hgt
        && valid_eyr
        && valid_hcl
    })
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_part2() {
    let test_input = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    let parsed = generator(&test_input);
    let result = part2(&parsed);
    assert_eq!(result, 4)
  }
}
