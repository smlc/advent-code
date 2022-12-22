use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1},
    combinator::{map, opt},
    IResult, sequence::{preceded, tuple},
};
use std::{collections::HashMap, cell::RefCell, rc::Rc, fmt, borrow::Borrow};

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Default)]
struct Node {
    size: usize,
    children: HashMap<String, NodeHandle>,
    parent:  Option<NodeHandle>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0
    }

    fn total_size(&self) -> u32 {
        self.children
            .values()
            .map(|rc|rc.borrow().total_size())
            .sum::<u32>()
            + self.size as u32
    }
}

#[derive(Debug)]
struct Command<'a>(&'a str, &'a str, Option<&'a str>);

#[derive(Debug)]
struct CommandOutput<'b>(&'b str, &'b str);

#[derive(Debug)]
enum Line<'c> {
    Command(Command<'c>),
    CommandOutput(CommandOutput<'c>)
}

fn main() {

    part_one();
}

fn part_one() {
    let input_string = include_str!("input07.txt");

    let root = Rc::new(RefCell::new(Node::default()));
    
    let mut node = root.clone();

    for line in input_string.lines() {
        //let (remaining, (f, s, t)) = parse_command("$ cd /").unwrap();
        
        let line = parse_line(line).unwrap();
        match line.1  {
            Line::Command(cmd) => match cmd.2 {
                Some("/") => {
                    
                }
                Some("..") => {
                    let parent = node.borrow().parent.clone().unwrap();
                    node = parent;
                }
                Some(&_) => {
                    let child = node.borrow_mut().children.entry(cmd.2.unwrap().to_string()).or_default().clone();
                    node = child;
                }
                None => {
                    println!("No handle");
                }
                

            },
            Line::CommandOutput(output) => {
                if output.0 == "dir" {
                    let dir = node.borrow_mut().children.entry(output.1.to_string()).or_default().clone();
                    dir.borrow_mut().parent = Some(node.clone());
                } else {
                    let file = node.borrow_mut().children.entry(output.1.to_string()).or_default().clone();
                    file.borrow_mut().size = output.0.parse().unwrap();
                    file.borrow_mut().parent = Some(node.clone());
                }
            }
        }
    }

    println!("{:?}", root.clone().borrow().tot);
}

fn parse_command(cmd : &str) -> IResult<&str, Command> {

    let to_command = |(f, s, t)| Command(f,s,t);
    let parser = tuple((
            tag("$"),
            preceded(tag(" "), alpha1),
            opt(preceded(tag(" "), take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c))))
    ));
    map(parser, to_command)(cmd)
}

fn parse_command_output(output: &str) -> IResult<&str, CommandOutput> {
    map(
        tuple((
            alphanumeric1,
            preceded(tag(" "), take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz.".contains(c)))
        )),
        |(f, s)| CommandOutput(f, s)
    )(output)
}

fn parse_line(line: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_command_output, Line::CommandOutput),
    ))(line)
}


