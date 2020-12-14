#[aoc_generator(day13, part1)]
pub fn generator_part1(input: &str) -> (u32, Vec<u32>) {
    let (earliest_departure, raw_lines) = input.split_once("\n").unwrap();
    let lines = raw_lines
        .split(',')
        .filter(|s| *s != "x")
        .map(|bus_line| bus_line.parse::<u32>().unwrap())
        .collect();
    (earliest_departure.parse::<u32>().unwrap(), lines)
}

#[aoc_generator(day13, part2)]
pub fn generator_part2(input: &str) -> (Vec<(i64, i64)>, i64) {
    let (_, raw_lines) = input.split_once("\n").unwrap();
    // Equations has the form of
    //[(a,b)] which represents
    //t+a ≅ 0 mod(b)
    //We then rewrite it as
    // t ≅ -a mod (b)
    let mut equations: Vec<(i64, i64)> = Vec::new();
    let mut counter = 0;
    for line in raw_lines.split(",") {
        match line.parse::<i64>() {
            Ok(n) => equations.push((-counter, n)),
            Err(_) => (),
        }
        counter += 1;
    }
    // and finally take the smallest a and add that to each so we get
    // t+a_l ≅ -a+a_l mod(b)
    let a_l = equations.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    for (a, b) in equations.iter_mut() {
        *a -= a_l;
        *a %= *b;
    }

    (equations, -a_l)
}

#[aoc(day13, part1)]
pub fn part1(input: &(u32, Vec<u32>)) -> u32 {
    let shortest_wait = input
        .1
        .iter()
        .map(|bus_line| (bus_line, bus_line * (input.0 / bus_line + 1) - input.0))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    shortest_wait.0 * shortest_wait.1
}

#[aoc(day13, part2)]
pub fn part2(input: &(Vec<(i64, i64)>, i64)) -> i64 {
    chinese_remainder_theorem(&input.0) - input.1
}

// Adapted from Rosetta code for crt
fn chinese_remainder_theorem(pairs: &[(i64, i64)]) -> i64 {
    let (_, n): (Vec<i64>, Vec<i64>) = pairs.iter().cloned().unzip();
    let n_sum: i64 = n.iter().product();

    let mut sum = 0;

    for (a, n) in pairs {
        let p = n_sum / n;
        sum += a * mod_inv(p, *n).unwrap() * p;
    }
    sum % n_sum
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
