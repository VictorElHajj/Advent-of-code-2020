use std::{collections::VecDeque, unreachable};

#[derive(Debug)]
pub enum Expr {
    BinOp(BinOp),
    Num(u64),
}

#[derive(Debug)]
pub enum Token {
    BinOp(BinOp),
    LeftParens,
    RightParens,
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Mul,
}

fn op_precedence_part_1(bo: &BinOp) -> u8 {
    match bo {
        BinOp::Add => 1,
        BinOp::Mul => 1,
    }
}

fn op_precedence_part_2(bo: &BinOp) -> u8 {
    match bo {
        BinOp::Add => 2,
        BinOp::Mul => 1,
    }
}

fn add_right_parens(output_queue: &mut VecDeque<Expr>, op_stack: &mut Vec<Token>) {
    loop {
        match op_stack.pop().unwrap() {
            Token::LeftParens => break,
            Token::BinOp(v) => output_queue.push_back(Expr::BinOp(v)),
            _ => unreachable!(),
        }
    }
}

fn add_op(
    bo: BinOp,
    precedence: &dyn Fn(&BinOp) -> u8,
    output_queue: &mut VecDeque<Expr>,
    op_stack: &mut Vec<Token>,
) {
    match op_stack.pop() {
        Some(Token::BinOp(v)) => {
            if precedence(&bo) > precedence(&v) {
                op_stack.push(Token::BinOp(v));
            } else if precedence(&bo) == precedence(&v) {
                output_queue.push_back(Expr::BinOp(v));
            } else {
                output_queue.push_back(Expr::BinOp(v));
                loop {
                    match op_stack.pop() {
                        Some(Token::BinOp(v)) => output_queue.push_back(Expr::BinOp(v)),
                        Some(Token::LeftParens) => {
                            op_stack.push(Token::LeftParens);
                            break;
                        }
                        None => break,
                        x => panic!("Unexpected token in opstack: {:?}", x),
                    }
                }
            }
        }
        Some(v) => {
            op_stack.push(v);
        }
        None => (),
    }
    op_stack.push(Token::BinOp(bo))
}

fn parse_ast(line: &str, precedence: &dyn Fn(&BinOp) -> u8) -> VecDeque<Expr> {
    let mut output_queue: VecDeque<Expr> = VecDeque::new();
    let mut op_stack: Vec<Token> = Vec::new();
    for b in line.bytes() {
        match b as char {
            // If literal add to output queue
            '0'..='9' => output_queue.push_back(Expr::Num(b as u64 - ('0' as u64))),
            // If operator
            '+' => add_op(BinOp::Add, precedence, &mut output_queue, &mut op_stack),
            '*' => add_op(BinOp::Mul, precedence, &mut output_queue, &mut op_stack),
            '(' => op_stack.push(Token::LeftParens),
            ')' => add_right_parens(&mut output_queue, &mut op_stack),
            ' ' => continue,
            _ => unreachable!(),
        }
    }
    //for op in op_stack.iter() {
    loop {
        match op_stack.pop() {
            Some(Token::BinOp(v)) => output_queue.push_back(Expr::BinOp(v)),
            None => break,
            x => panic!("Unexpected token in opstack: {:?}", x),
        }
    }
    output_queue
}

fn eval_rpn(rpn: &mut VecDeque<Expr>) -> u64 {
    let mut stack: Vec<u64> = Vec::new();
    for exp in rpn {
        match exp {
            Expr::Num(v) => stack.push(*v),
            Expr::BinOp(BinOp::Add) => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x + y);
            }
            Expr::BinOp(BinOp::Mul) => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x * y);
            }
        }
    }
    stack.pop().unwrap()
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u64 {
    let mut rpns: Vec<VecDeque<Expr>> = input
        .lines()
        .map(|l| parse_ast(l, &op_precedence_part_1))
        .collect();
    rpns.iter_mut().map(|rpn| eval_rpn(rpn)).sum()
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> u64 {
    let mut rpns: Vec<VecDeque<Expr>> = input
        .lines()
        .map(|l| parse_ast(l, &op_precedence_part_2))
        .collect();
    rpns.iter_mut().map(|rpn| eval_rpn(rpn)).sum()
}
