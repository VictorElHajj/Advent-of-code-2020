use std::cell::Cell;

use Instruction::*;

#[derive(Debug)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

pub struct Computer {
    imem: Vec<Instruction>,
    pc: Cell<usize>,
    acc: Cell<i32>,
}

impl Computer {
    fn inc_pc(&self) {
        self.pc.set(self.pc.get() + 1);
    }

    fn update_pc(&self, v: &i32) {
        self.pc.set((self.pc.get() as i32 + v) as usize);
    }

    fn update_acc(&self, v: &i32) {
        self.acc.set(self.acc.get() + v);
    }
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Computer {
    let instructions = input
        .lines()
        .map(|line| {
            let (ins, val) = line.split_once(' ').unwrap();
            let val = val.parse::<i32>().unwrap();
            match ins {
                "acc" => ACC(val),
                "jmp" => Instruction::JMP(val),
                "nop" => Instruction::NOP(val),
                _ => unreachable!(),
            }
        })
        .collect();
    Computer {
        imem: instructions,
        pc: Cell::new(0),
        acc: Cell::new(0),
    }
}

#[aoc(day8, part1)]
pub fn part1(computer: &Computer) -> i32 {
    let mut visited: Vec<bool> = vec![false; computer.imem.len()];
    loop {
        let pc = computer.pc.get();
        visited[pc] = true;
        match computer.imem.get(computer.pc.get()).unwrap() {
            ACC(v) => {
                computer.update_acc(v);
                computer.inc_pc();
            }
            JMP(v) => {
                computer.update_pc(v);
                if visited[computer.pc.get()] {
                    return computer.acc.get();
                }
            }
            NOP(_) => {
                computer.inc_pc();
            }
        }
    }
}

fn compute_with_flip(computer: &Computer, flip: usize) -> Result<i32, &str> {
    computer.pc.set(0);
    computer.acc.set(0);
    let mut visited: Vec<bool> = vec![false; computer.imem.len()];
    loop {
        let pc = computer.pc.get();
        if pc == computer.imem.len() {
            return Ok(computer.acc.get());
        }
        visited[pc] = true;
        match computer.imem.get(computer.pc.get()).unwrap() {
            ACC(v) => {
                computer.update_acc(v);
                computer.inc_pc();
            }
            JMP(_) if pc == flip => computer.inc_pc(),
            JMP(v) => {
                computer.update_pc(v);
                if computer.pc.get() < computer.imem.len() && visited[computer.pc.get()] {
                    return Err("Loop detected");
                }
            }
            NOP(v) if pc == flip => {
                computer.update_pc(v);
                if computer.pc.get() < computer.imem.len() && visited[computer.pc.get()] {
                    return Err("Loop detected");
                }
            }
            NOP(_) => {
                computer.inc_pc();
            }
        }
    }
}

#[aoc(day8, part2)]
pub fn part2(computer: &Computer) -> Result<i32, &str> {
    for (i, v) in computer.imem.iter().enumerate() {
        match v {
            ACC(_) => continue,
            NOP(_) | JMP(_) => match compute_with_flip(computer, i) {
                Ok(acc) => return Ok(acc),
                Err(_) => {
                    continue;
                }
            },
        }
    }
    Err("Could not solve")
}
