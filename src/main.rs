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

#[derive(Debug, PartialEq)]
struct Interpreter {
    memory: Vec<u8>,
    pointer: usize
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
        Some(CommandType::While) =>
            Some(
                Node {
                    command_type: CommandType::While,
                    left: (
                            match parse(v2) {
                                Some(node) => Some(Box::new(node)),
                                None => None
                            }
                        ),
                    right: (
                            match parse(v2) {
                                Some(node) => Some(Box::new(node)),
                                None => None
                            }
                        )
                }
            ),
        Some(CommandType::End) =>
            Some(
                Node {
                    command_type: CommandType::End,
                    left: None,
                    right: None
                }
            ),
        Some(command_type) =>
            Some(
                Node {
                    command_type,
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

fn execute(interpreter: &mut Interpreter, node: Node) -> io::Result<()> {
    match node.command_type {
        CommandType::Inc => {
            interpreter.memory[interpreter.pointer] += 1;
            match node.left {
                Some(left) => {
                    execute(interpreter, *left)
                },
                None => Ok(())
            }
        },
        CommandType::Dec => {
            interpreter.memory[interpreter.pointer] -= 1;
            match node.left {
                Some(left) => {
                    execute(interpreter, *left)
                },
                None => Ok(())
            }
        },
        CommandType::PInc => {
            interpreter.pointer += 1;
            match node.left {
                Some(left) => {
                    execute(interpreter, *left)
                },
                None => Ok(())
            }
        },
        CommandType::PDec => {
            interpreter.pointer -= 1;
            match node.left {
                Some(left) => {
                    execute(interpreter, *left)
                },
                None => Ok(())
            }
        },
        CommandType::While => {
            if interpreter.memory[interpreter.pointer] == 0 {
                match node.right {
                    Some(right) => {
                        execute(interpreter, *right)
                    },
                    None => Ok(())
                }
            } else {
                match node.left {
                    Some(left) => {
                        execute(interpreter, *left)
                    },
                    None => Ok(())
                }
            }
        }
        _ => {
            Ok(())
        }
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("test.bf")?;
    let mut contents = String::new();
    let mut interpreter: Interpreter = Interpreter {
        memory: vec![0; 100],
        pointer: 0
    };

    file.read_to_string(&mut contents)?;
    let mut v2: Vec<CommandType> = contents.chars().filter_map(char_to_command_type).collect::<Vec<CommandType>>();
    v2.reverse();
    match parse(&mut v2) {
        Some(ast) => {
            println!("{:?}", ast);
            execute(&mut interpreter, ast);
        }
        None => {

        }
    }
    println!("{:?}", interpreter.memory);
    Ok(())
}