use std::cell::Cell;

#[derive(Debug)]
pub struct Rule {
    pos: Cell<u16>,
    min1: u16,
    max1: u16,
    min2: u16,
    max2: u16,
}

impl Rule {
    fn from_str(input: &str) -> Rule {
        let mut iter = input.bytes();
        let min1 = iter
            .by_ref()
            .skip_while(|b| *b as char != ':')
            .skip(2)
            .take_while(|b| *b as char != '-')
            .fold(0, |tot, c| tot * 10 + (c as u16 - '0' as u16));
        let max1 = iter
            .by_ref()
            .take_while(|b| *b as char != ' ')
            .fold(0, |tot, c| tot * 10 + (c as u16 - '0' as u16));
        let min2 = iter
            .by_ref()
            .skip_while(|b| *b as char != ' ')
            .skip(1)
            .take_while(|b| *b as char != '-')
            .fold(0, |tot, c| tot * 10 + (c as u16 - '0' as u16));
        let max2 = iter
            .by_ref()
            .take_while(|b| *b as char != '\n')
            .fold(0, |tot, c| tot * 10 + (c as u16 - '0' as u16));
        Rule {
            pos: Cell::new(0),
            min1,
            max1,
            min2,
            max2,
        }
    }
    fn is_valid(&self, val: &u16) -> bool {
        (*val >= self.min1 && *val <= self.max1) || (*val >= self.min2 && *val <= self.max2)
    }
}

type Ticket = Vec<u16>;
fn ticket_from_str(input: &str) -> Ticket {
    input
        .split(',')
        .map(|n| {
            n.bytes()
                .fold(0, |tot, c| tot * 10 + (c as u16 - '0' as u16))
        })
        .collect()
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let mut sections = input.split("\n\n");
    let raw_rules = sections.next().unwrap();
    let rules: Vec<Rule> = raw_rules.lines().map(|l| Rule::from_str(l)).collect();
    let raw_your_ticket = sections.next().unwrap();
    let your_ticket: Ticket = ticket_from_str(raw_your_ticket.lines().skip(1).next().unwrap());
    let raw_nearby_tickets = sections.next().unwrap();
    let nearby_tickets: Vec<Ticket> = raw_nearby_tickets
        .lines()
        .skip(1)
        .map(|l| ticket_from_str(l))
        .collect();
    (rules, your_ticket, nearby_tickets)
}

#[aoc(day16, part1)]
pub fn part1((rules, _, nearby_tickets): &(Vec<Rule>, Ticket, Vec<Ticket>)) -> u16 {
    let mut invalid_sum = 0;
    for ticket in nearby_tickets {
        for value in ticket {
            if !rules.iter().any(|rule| rule.is_valid(value)) {
                invalid_sum += value;
            }
        }
    }
    invalid_sum
}

#[aoc(day16, part2)]
pub fn part2((rules, your_ticket, nearby_tickets): &(Vec<Rule>, Ticket, Vec<Ticket>)) -> u64 {
    let len = your_ticket.len();

    // begin by repeating part in in iterator style to filter out invalid tickets
    let valid_tickets: Vec<Ticket> = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|value| rules.iter().any(|rule| rule.is_valid(value)))
        })
        .cloned()
        .collect();

    // now we find which rules are valid for which columns
    let mut valid_for: Vec<Vec<u16>> = vec![Vec::new(); len];
    for (r, rule) in rules.iter().enumerate() {
        for i in 0..len {
            if valid_tickets.iter().all(|ticket| rule.is_valid(&ticket[i])) {
                valid_for[r].push(i as u16);
            }
        }
    }

    // the rule with only one valid position first, then the next without the first and so on
    // when sorting valid_for[i] no longer means the ith rule, so we zip with 0..
    let mut valid_for_with_index: Vec<(&Vec<u16>, usize)> = valid_for.iter().zip(0..).collect();
    valid_for_with_index.sort_by(|a, b| a.0.len().cmp(&b.0.len()));

    let mut taken = vec![false; len];
    for (valid_pos, r) in valid_for_with_index {
        for pos in valid_pos {
            if !taken[*pos as usize] {
                taken[*pos as usize] = true;
                rules[r].pos.set(*pos);
                break;
            }
        }
    }

    // the rules for departure are rulles 0..6
    rules[0..6]
        .iter()
        .map(|rule| your_ticket[rule.pos.get() as usize] as u64)
        .product()
}
