
use std::{fmt::{Formatter, self}, collections::VecDeque};
use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, opt, map_res},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};
struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

struct Piles(Vec<Vec<Crate>>);
impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}

impl Piles {
    fn apply(&mut self, ins: Instruction) {
        for _ in 0..ins.quantity {
            let el = self.0[ins.src].pop().unwrap();
            self.0[ins.dst].push(el);
        }
    }
}

fn main() {
    // https://fasterthanli.me/series/advent-of-code-2022/part-5
    // https://blog.adamchalmers.com/nom-chars/

    let mut lines = include_str!("input05.txt").lines();

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();

    let mut piles = Piles(transpose(crate_lines));
    println!("{piles:?}");

    assert!(lines.next().unwrap().is_empty()); //Consume the empty line
    
    let instructions: Vec<_> = lines
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
        .collect();
    
    for ins in instructions {
        piles.apply(ins);
    }

    println!("{piles:?}");

    println!(
        "answer = {}",
        piles.0.iter().map(|pile| pile.last().unwrap()).join("")
    );
}

fn parse_crate(c: &str) -> IResult<&str, Crate> {
    let first_char = |parser_output: &str| Crate(parser_output.chars().next().unwrap());
    let parser = delimited(tag("["), take(1_usize), tag("]"));
    map(parser, first_char)(c)
}

fn parse_hole(line : &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(line)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(line: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut remaining, parsed_element) = parse_crate_or_hole(line)?;
    let mut parsed_elements = vec![parsed_element];

    loop {
        let (next, parsed_element) = opt(preceded(tag(" "), parse_crate_or_hole))(remaining)?;
        match parsed_element {
            Some(c) => parsed_elements.push(c),
            None => break,
        }
        remaining = next;
    }

    Ok((remaining, parsed_elements))
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}

fn transpose<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
