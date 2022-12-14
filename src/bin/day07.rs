use std::{cell::RefCell, ops::Deref};
use std::rc::Rc;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1},
    combinator::{map, opt},
    IResult, sequence::{preceded, tuple},
};


#[derive(PartialEq)]
struct Node {
  pub name: Rc<String>,
  pub size: u32,
  pub children: Vec<Rc<RefCell<Node>>>,
  pub parent: Option<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new(name: &str, size: u32) -> Node {
      return Node {
        name: Rc::new(name.to_string()),
        size: size,
        children: vec![],
        parent: None,
      };
    }


  pub fn add_child(&mut self, new_node: Rc<RefCell<Node>>) {
    self.children.push(new_node);
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
    let line = parse_line("14848514 b.txt").unwrap();
    
    println!("first {:?}", line.1);
    match line.1  {
        Line::Command(cmd) => println!("first {}, second {}", cmd.0, cmd.1),
        Line::CommandOutput(output) => println!("first {}, second {}", output.0, output.1)
    }

    //let (rest, (f1, s2)) =  parse_command_output("$ cd ..").unwrap();
    //println!("first {}, second {}", f1, s2);
}

fn part_one() {
    let input_string = include_str!("input07.txt");

    let root = Rc::new(RefCell::new(Node::new("/", 0)));
    
    let mut current_directory = Rc::clone(&root);
    for line in input_string.lines() {
        //let (remaining, (f, s, t)) = parse_command("$ cd /").unwrap();
        
        let line = parse_line(line).unwrap();
        match line.1  {
            Line::Command(cmd) => {

                
                if cmd.1 == "cd" &&  cmd.2.unwrap() == ".."{
                    if current_directory.borrow().parent.is_some(){
                        let current_clone = Rc::clone(&current_directory);
                        current_directory = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                    }
                }

                
            },
            Line::CommandOutput(output) => println!("first {}, second {}", output.0, output.1)
        }
    }
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


