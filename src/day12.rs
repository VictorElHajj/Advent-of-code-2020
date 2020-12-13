pub enum Action {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|line| {
            let len = line
                .bytes()
                .skip(1)
                .fold(0, |tot, c| tot * 10 + (c as i32 - '0' as i32));
            match line.chars().next().unwrap() {
                'N' => Action::N(len),
                'S' => Action::S(len),
                'E' => Action::E(len),
                'W' => Action::W(len),
                'L' => Action::L(len),
                'R' => Action::R(len),
                'F' => Action::F(len),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(actions: &Vec<Action>) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut deg = 0;
    for action in actions {
        match action {
            Action::N(n) => pos.1 += n,
            Action::S(n) => pos.1 -= n,
            Action::E(n) => pos.0 += n,
            Action::W(n) => pos.0 -= n,
            Action::L(n) => deg += n,
            Action::R(n) => deg -= n,
            Action::F(n) => {
                let angle = ((deg % 360) + 360) % 360;
                if angle == 0 {
                    pos.0 += n;
                } else if angle == 90 {
                    pos.1 += n;
                } else if angle == 180 {
                    pos.0 -= n;
                } else if angle == 270 {
                    pos.1 -= n;
                }
            }
        }
    }
    pos.0.abs() + pos.1.abs()
}

#[aoc(day12, part2)]
pub fn part2(actions: &Vec<Action>) -> i32 {
    let mut waypoint_pos: (i32, i32) = (10, 1);
    let mut ship_pos: (i32, i32) = (0, 0);
    for action in actions {
        match action {
            Action::N(n) => waypoint_pos.1 += n,
            Action::S(n) => waypoint_pos.1 -= n,
            Action::E(n) => waypoint_pos.0 += n,
            Action::W(n) => waypoint_pos.0 -= n,
            Action::L(n) => {
                let angle = ((n % 360) + 360) % 360;
                let old_pos = waypoint_pos;
                if angle == 0 {
                    waypoint_pos.0 = old_pos.0;
                    waypoint_pos.1 = old_pos.1;
                } else if angle == 90 {
                    waypoint_pos.0 = -old_pos.1;
                    waypoint_pos.1 = old_pos.0;
                } else if angle == 180 {
                    waypoint_pos.0 = -old_pos.0;
                    waypoint_pos.1 = -old_pos.1;
                } else if angle == 270 {
                    waypoint_pos.0 = old_pos.1;
                    waypoint_pos.1 = -old_pos.0;
                }
            }
            Action::R(n) => {
                let angle = ((-n % 360) + 360) % 360;
                let old_pos = waypoint_pos;
                if angle == 0 {
                    waypoint_pos.0 = old_pos.0;
                    waypoint_pos.1 = old_pos.1;
                } else if angle == 90 {
                    waypoint_pos.0 = -old_pos.1;
                    waypoint_pos.1 = old_pos.0;
                } else if angle == 180 {
                    waypoint_pos.0 = -old_pos.0;
                    waypoint_pos.1 = -old_pos.1;
                } else if angle == 270 {
                    waypoint_pos.0 = old_pos.1;
                    waypoint_pos.1 = -old_pos.0;
                }
            }
            Action::F(n) => {
                ship_pos.0 += waypoint_pos.0 * n;
                ship_pos.1 += waypoint_pos.1 * n;
            }
        }
    }
    ship_pos.0.abs() + ship_pos.1.abs()
}
