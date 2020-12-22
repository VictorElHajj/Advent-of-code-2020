use std::collections::HashMap;

use Cube::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cube {
    Active,
    Inactive,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Dimension {
    map: HashMap<(i16, i16, i16), Cube>,
}

type Dimension4 = HashMap<(i16, i16, i16, i16), Cube>;

#[aoc_generator(day17, part1)]
pub fn generator(input: &str) -> Dimension {
    let mut hm: HashMap<(i16, i16, i16), Cube> = HashMap::new();
    for (y, line) in input.as_bytes().chunks(9).enumerate() {
        // Change to 8 for real input
        for (x, b) in line.iter().enumerate() {
            let cube = match *b as char {
                '.' => Inactive,
                '#' => Active,
                '\n' => continue,
                _ => unreachable!(),
            };
            hm.insert((x as i16, y as i16, 0), cube);
        }
    }
    Dimension { map: hm }
}

#[aoc_generator(day17, part2)]
pub fn generator2(input: &str) -> Dimension4 {
    let mut hm: HashMap<(i16, i16, i16, i16), Cube> = HashMap::new();
    for (y, line) in input.as_bytes().chunks(9).enumerate() {
        // Change to 8 for real input
        for (x, b) in line.iter().enumerate() {
            let cube = match *b as char {
                '.' => Inactive,
                '#' => Active,
                '\n' => continue,
                _ => unreachable!(),
            };
            hm.insert((x as i16, y as i16, 0, 0), cube);
        }
    }
    hm
}

fn neighbors(hm: &HashMap<(i16, i16, i16), Cube>, (x, y, z): &(i16, i16, i16)) -> i16 {
    let mut sum = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                let pos = (*x + dx, *y + dy, *z + dz);
                if pos != (*x, *y, *z) {
                    match hm.get(&pos) {
                        Some(Active) => sum += 1,
                        Some(Inactive) => continue,
                        None => continue,
                    }
                }
            }
        }
    }
    sum
}

fn neighbors2(
    hm: &HashMap<(i16, i16, i16, i16), Cube>,
    (x, y, z, w): &(i16, i16, i16, i16),
) -> i16 {
    let mut sum = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    let pos = (*x + dx, *y + dy, *z + dz, *w + dw);
                    if pos != (*x, *y, *z, *w) {
                        match hm.get(&pos) {
                            Some(Active) => sum += 1,
                            Some(Inactive) => continue,
                            None => continue,
                        }
                    }
                }
            }
        }
    }
    sum
}

#[aoc(day17, part1)]
pub fn part1(input: &Dimension) -> u32 {
    let mut old_hm = input.map.clone();
    let mut sum = 0;

    //println!();
    //println!("cycle: 0");
    // for y in 0..8 {
    //     for x in 0..8 {
    //         match old_hm.get(&(x, y, 0)) {
    //             Some(Active) => print!("#"),
    //             Some(Inactive) => print!("."),
    //             None => unreachable!(),
    //         };
    //     }
    //     //println!();
    // }

    for i in 1i16..=6i16 {
        // Change to 6 for real input
        //println!();
        //println!("cycle: {}", i);
        let mut new_hm = old_hm.clone();
        for dimension in (0 - i)..(i + 1) {
            //println!();
            //println!("z: {}", dimension);
            for y in (0 - i)..(8 + i) {
                for x in (0 - i)..(8 + i) {
                    let pos = (x, y, dimension);
                    let n_count = neighbors(&old_hm, &pos);
                    if old_hm.get(&pos) == Some(&Active) {
                        if n_count == 2 || n_count == 3 {
                            new_hm.insert(pos, Active);
                            //print!("#");
                            if i == 6 {
                                sum += 1;
                            }
                        } else {
                            new_hm.insert(pos, Inactive);
                            //print!(".");
                        }
                    } else {
                        if n_count == 3 {
                            new_hm.insert(pos, Active);
                            //print!("#");
                            if i == 6 {
                                sum += 1;
                            }
                        } else {
                            new_hm.insert(pos, Inactive);
                            //print!(".");
                        }
                    }
                }
                //println!("");
            }
        }
        old_hm = new_hm;
    }
    sum
}

#[aoc(day17, part2)]
pub fn part2(input: &Dimension4) -> u32 {
    let mut old_hm = input.clone();
    let mut sum = 0;
    for i in 1i16..=6i16 {
        let mut new_hm = old_hm.clone();
        for z in (0 - i)..(i + 1) {
            for w in (0 - i)..(i + 1) {
                for y in (0 - i)..(8 + i) {
                    for x in (0 - i)..(8 + i) {
                        let pos = (x, y, z, w);
                        let n_count = neighbors2(&old_hm, &pos);
                        if old_hm.get(&pos) == Some(&Active) {
                            if n_count == 2 || n_count == 3 {
                                new_hm.insert(pos, Active);
                                if i == 6 {
                                    sum += 1;
                                }
                            } else {
                                new_hm.insert(pos, Inactive);
                            }
                        } else {
                            if n_count == 3 {
                                new_hm.insert(pos, Active);
                                if i == 6 {
                                    sum += 1;
                                }
                            } else {
                                new_hm.insert(pos, Inactive);
                            }
                        }
                    }
                }
            }
        }
        old_hm = new_hm;
    }
    sum
}
