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
    right: Option<Box<Node>>,
}

#[derive(Debug, PartialEq)]
struct Interpreter {
    memory: Vec<u8>,
    pointer: usize,
    current_node: Option<Box<Node>>
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


fn execute(interpreter: Interpreter) -> Option<Interpreter> {
    let mut memory = interpreter.memory;
    let pointer = interpreter.pointer;
     interpreter.current_node.and_then(|current_node|
         match (*current_node).command_type {
             CommandType::Inc => {
                 memory[pointer] += 1;
                 Some(Interpreter {
                     memory,
                     pointer,
                     current_node: (*current_node).left
                 })
             },
             CommandType::Dec => {
                 memory[pointer] -= 1;
                 Some(Interpreter {
                     memory,
                     pointer,
                     current_node: (*current_node).left
                 })
             },
             CommandType::PInc => {
                 Some(Interpreter {
                     memory,
                     pointer: pointer + 1,
                     current_node: (*current_node).left
                 })
             },
             CommandType::PDec => {
                 Some(Interpreter {
                     memory,
                     pointer: pointer - 1,
                     current_node: (*current_node).left
                 })
             },
             CommandType::While => {
                 if memory[pointer] == 0 {
                     Some(Interpreter {
                         memory,
                         pointer,
                         current_node: (*current_node).right
                     })
                 } else {
                     Some(Interpreter {
                         memory,
                         pointer,
                         current_node: (*current_node).left
                     })
                 }
             },
            _ => None
         }
     )
}

fn main() -> io::Result<()> {
    let mut file = File::open("test.bf")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let mut v2: Vec<CommandType> = contents.chars().filter_map(char_to_command_type).collect::<Vec<CommandType>>();
    v2.reverse();
    let mut optionInterpreter: Option<Interpreter> = parse(&mut v2).and_then(|ast|
        Some(Interpreter {
            memory: vec![0; 100],
            pointer: 0,
            current_node: Some(Box::new(ast))
        })
    );
    while optionInterpreter.is_some() {
        optionInterpreter = optionInterpreter.and_then(execute);
        println!("{:?}", optionInterpreter);
    }
    Ok(())
}