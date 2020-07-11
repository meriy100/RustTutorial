use std::fs::File;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
enum CommandType {
    Inc, Dec, PInc, PDec, Get, Set, While, End
}

#[derive(Debug, PartialEq)]
struct Node {
    command_type: CommandType,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

fn char_to_command_type(c: char) -> Option<CommandType> {
    return match c {
        '+' => Some(CommandType::Inc),
        '-' => Some(CommandType::Dec),
        '>' => Some(CommandType::PInc),
        '<' => Some(CommandType::PDec),
        '[' => Some(CommandType::While),
        ']' => Some(CommandType::End),
        '.' => Some(CommandType::Set),
        ',' => Some(CommandType::Get),
        _ => None
    }
}

fn parse(v2: &mut Vec<CommandType>) -> Option<Node> {
    return match v2.pop() {
        Some(command_type) =>
            Some(
                Node {
                    command_type: command_type,
                    left: (
                            match parse(v2) {
                                Some(node) => Some(Box::new(node)),
                                None => None
                            }
                        ),
                    right: None
                }
            ),
        None => None
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("test.bf")?;
    let mut contents = String::new();
    let mut memory: Vec<u8> = vec![0; 100];

    file.read_to_string(&mut contents)?;
    let mut v2: Vec<CommandType> = contents.chars().filter_map(char_to_command_type).collect::<Vec<CommandType>>();
    v2.reverse();
    // match v2.pop() {
    //     Some(command_type) => {
    //        let mut ast: Node = Node { command_type: command_type, left: None, right: None };
    //         match v2.pop() {
    //             Some(command_type1) => {
    //                 ast.left = Some(Box::new(Node { command_type: command_type1, left: None, right: None }));
    //
    //             },
    //             None => {}
    //         }
    //     },
    //     None => {}
    // }
    println!("{:?}", parse(&mut v2));
    println!("{:?}", memory);
    Ok(())
}