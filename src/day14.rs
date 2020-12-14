use std::collections::HashMap;

pub struct BitMask {
    ones: u64,
    zeroes: u64,
    floating: u64,
}

impl BitMask {
    fn new() -> BitMask {
        let ones = 0;
        let zeroes = 0;
        let floating = 0;
        BitMask {
            ones,
            zeroes,
            floating,
        }
    }
}

pub struct MemAssign {
    mem_loc: u64,
    value: u64,
}
pub enum Instruction {
    Mask(BitMask),
    Mem(MemAssign),
}

#[aoc_generator(day14)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars();
            match iter
                .by_ref()
                .take_while(|c| *c != ' ' && *c != '[')
                .collect::<String>()
                .as_str()
            {
                "mask" => {
                    let mut ones: u64 = 0;
                    let mut floating: u64 = 0;
                    let mut zeroes: u64 = !0;
                    for (i, b) in iter.by_ref().skip(2).enumerate() {
                        match b as char {
                            '1' => ones |= 1 << (35 - i),
                            '0' => zeroes &= !(1 << (35 - i)),
                            'X' => floating |= 1 << (35 - i),
                            _ => unreachable!(),
                        }
                    }
                    Instruction::Mask(BitMask {
                        ones,
                        zeroes,
                        floating,
                    })
                }
                "mem" => {
                    let mem_loc = iter
                        .by_ref()
                        .take_while(|c| *c != ']')
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                    let value = iter
                        .by_ref()
                        .skip_while(|c| *c == ' ' || *c == '=' || *c == ']')
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                    Instruction::Mem(MemAssign { mem_loc, value })
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(instructions: &Vec<Instruction>) -> u64 {
    let mut curr_mask: &BitMask = &BitMask::new();
    let mut memory: HashMap<u64, u64> = HashMap::with_capacity(1000);
    for instruction in instructions {
        match instruction {
            Instruction::Mask(bm) => {
                curr_mask = bm;
            }
            Instruction::Mem(ma) => {
                let mut assignment: u64 = ma.value;
                assignment |= curr_mask.ones;
                assignment &= curr_mask.zeroes;
                memory.insert(ma.mem_loc, assignment);
            }
        }
    }
    memory.values().sum()
}

#[aoc(day14, part2)]
pub fn part2(instructions: &Vec<Instruction>) -> u64 {
    let mut curr_mask: &BitMask = &BitMask::new();
    let mut memory: HashMap<u64, u64> = HashMap::with_capacity(1000);
    for instruction in instructions {
        match instruction {
            Instruction::Mask(bm) => {
                curr_mask = bm;
            }
            Instruction::Mem(ma) => {
                let assignment: u64 = (ma.mem_loc | curr_mask.ones) & !curr_mask.floating;
                let mut deltas: Vec<u64> = vec![0];
                for bit in 0..36 {
                    if curr_mask.floating & (1 << bit) != 0 {
                        let old_delta = deltas.clone();
                        for delta in old_delta {
                            deltas.push(delta + (1 << bit));
                        }
                    }
                }
                for delta in deltas {
                    let memory_loc = assignment + delta;
                    memory.insert(memory_loc, ma.value);
                }
            }
        }
    }
    memory.values().sum()
}
