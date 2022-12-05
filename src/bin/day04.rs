use std::str::FromStr;
use anyhow::Result;
struct Task {
    start: usize,
    end: usize
}

impl Task {
    pub fn new(start: usize, end: usize) -> Self {
        return Task { start, end };
    }

    pub fn contains(&self, other: &Task) -> bool {
        return (self.start <= other.start && self.end >= other.end)
            || (other.start <= self.start && other.end >= self.end);
    }

    pub fn partial_contains(&self, other: &Task) -> bool {
        return self.start <= other.start && self.end >= other.start
            || self.end >= other.end && self.start <= other.end;
    }

}

impl FromStr for Task {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) =s.split_once('-').expect("Something wrong in the input");
        return Ok(Task{
            start: start.parse::<usize>()?,
            end: end.parse::<usize>()?
        });
    }
}
fn main() {
    let input: Vec<&str> = include_str!("input04.txt")
        .lines()
        .collect();
    //part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<&str>) {
    
    let total: usize = input
        .iter()
        .map(|line| {
            let (left, right) = line.split_once(',').expect("Parse ',' Something wrong with input");
             if left.parse::<Task>().unwrap().contains(&right.parse::<Task>().unwrap()) {
                return 1;
             }
             return 0;
        }).sum();
   println!("Total pair : {}", total);
}

fn part_two(input: &Vec<&str>) {
    
    let total: usize = input
        .iter()
        .map(|line| {
            let (left, right) = line.split_once(',').expect("Parse ',' Something wrong with input");
             if left.parse::<Task>().unwrap().partial_contains(&right.parse::<Task>().unwrap()) ||
             right.parse::<Task>().unwrap().partial_contains(&left.parse::<Task>().unwrap())  {
                return 1;
             }
             return 0;
        }).sum();
   println!("Total pair : {}", total);
}
