use Tile::*;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile {
    Empty,
    Occupied,
    Floor,
}

fn parse_tile(char: &char) -> Tile {
    match char {
        'L' => Empty,
        '.' => Floor,
        _ => unreachable!(),
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Seats {
    map: Vec<Vec<Tile>>,
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Seats {
    let map = input
        .lines()
        .map(|l| l.chars().map(|c| parse_tile(&c)).collect())
        .collect();
    Seats { map }
}

fn neighbors(map: &Vec<Vec<Tile>>, x: usize, y: usize) -> [Tile; 8] {
    let mut output = [Floor; 8];
    let neighbors: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    for (i, (dx, dy)) in neighbors.iter().enumerate() {
        let row = map.get(y + *dy as usize);
        match row {
            Some(r) => {
                let col = r.get(x + *dx as usize);
                match col {
                    Some(c) => {
                        output[i] = *c;
                    }
                    None => continue,
                }
            }
            None => continue,
        }
    }
    output
}

fn neighbors2(map: &Vec<Vec<Tile>>, x: usize, y: usize) -> [Tile; 8] {
    let mut output = [Floor; 8];
    let neighbors: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    for (dir, (dx, dy)) in neighbors.iter().enumerate() {
        for distance in 1.. {
            let row = map.get((y as i32 + distance * *dy) as usize);
            match row {
                Some(r) => {
                    let col = r.get((x as i32 + distance * *dx) as usize);
                    match col {
                        Some(Floor) => continue,
                        Some(tile) => {
                            output[dir] = *tile;
                            break;
                        }
                        None => break,
                    }
                }
                None => break,
            }
        }
    }
    output
}

// For future optimization, clone can be avoided by taking a mut reference
// Would still need a clone in part1 but here we could just split the loop
// in two loops, first check what values change and then change them.
fn run(seats: &Seats) -> Seats {
    let mut new_seats: Seats = seats.clone();
    for (y, row) in new_seats.map.iter_mut().enumerate() {
        for (x, col) in row.iter_mut().enumerate() {
            if *col != Floor {
                let n = neighbors(&seats.map, x, y);
                let adjecent_occupied = n.iter().filter(|t| **t == Occupied).count();
                if col == &Empty && adjecent_occupied == 0 {
                    *col = Occupied;
                } else if col == &Occupied && adjecent_occupied >= 4 {
                    *col = Empty;
                }
            }
        }
    }
    new_seats
}

fn run2(seats: &Seats) -> Seats {
    let mut new_seats: Seats = seats.clone();
    for (y, row) in new_seats.map.iter_mut().enumerate() {
        for (x, col) in row.iter_mut().enumerate() {
            if *col != Floor {
                let n = neighbors2(&seats.map, x, y);
                let visible_occupied = n.iter().filter(|t| **t == Occupied).count();
                if col == &Empty && visible_occupied == 0 {
                    *col = Occupied;
                } else if col == &Occupied && visible_occupied >= 5 {
                    *col = Empty;
                }
            }
            let test = match col {
                Empty => "L",
                Occupied => "#",
                Floor => ".",
            };
        }
    }
    new_seats
}

#[aoc(day11, part1)]
pub fn part1(seats: &Seats) -> usize {
    let mut old_seats = seats.clone();
    loop {
        let new_seats = run(&old_seats);
        if new_seats == old_seats {
            break;
        }
        old_seats = new_seats;
    }
    old_seats
        .map
        .iter()
        .map(|row| row.iter().filter(|col| **col == Occupied).count())
        .sum()
}

#[aoc(day11, part2)]
pub fn part2(seats: &Seats) -> usize {
    let mut old_seats = seats.clone();
    loop {
        let new_seats = run2(&old_seats);
        if new_seats == old_seats {
            break;
        }
        old_seats = new_seats;
    }
    old_seats
        .map
        .iter()
        .map(|row| row.iter().filter(|col| **col == Occupied).count())
        .sum()
}
