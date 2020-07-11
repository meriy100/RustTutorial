use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Debug, Clone, PartialEq)]
enum CommandType {
    Inc, Dec, PInc, PDec, Get, Set, While, End
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    command_type: CommandType,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug, PartialEq)]
struct Interpreter {
    memory: Vec<u8>,
    pointer: usize,
    current_node: Option<Box<Node>>,
    while_stack: Vec<Box<Node>>
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
    let mut while_stack = interpreter.while_stack;
     interpreter.current_node.and_then(|current_node|
         match (*current_node).command_type {
             CommandType::Inc => {
                 memory[pointer] = if memory[pointer] == 255 {
                     0 as u8
                 } else {
                     memory[pointer] + 1
                 };
                 Some(Interpreter {
                     memory,
                     pointer,
                     current_node: (*current_node).left,
                     while_stack
                 })
             },
             CommandType::Dec => {
                 memory[pointer] = if memory[pointer] == 0 {
                     255 as u8
                 } else {
                     memory[pointer] - 1
                 };
                 Some(Interpreter {
                     memory,
                     pointer,
                     current_node: (*current_node).left,
                     while_stack
                 })
             },
             CommandType::PInc => {
                 Some(Interpreter {
                     memory,
                     pointer: pointer + 1,
                     current_node: (*current_node).left,
                     while_stack
                 })
             },
             CommandType::PDec => {
                 Some(Interpreter {
                     memory,
                     pointer: pointer - 1,
                     current_node: (*current_node).left,
                     while_stack
                 })
             },
             CommandType::While => {
                 if memory[pointer] == 0 {
                     Some(Interpreter {
                         memory,
                         pointer,
                         current_node: (*current_node).right,
                         while_stack
                     })
                 } else {
                     while_stack.push(current_node.clone());
                     Some(Interpreter {
                         memory,
                         pointer,
                         current_node: (*current_node).left,
                         while_stack
                     })
                 }
             },
             CommandType::End => {
                 while_stack.pop().and_then(|while_node|
                     Some(Interpreter {
                         memory,
                         pointer,
                         current_node: Some(while_node),
                         while_stack
                     })
                 )
             }
             CommandType::Get => {
                 std::io::stdin()
                     .bytes()
                     .next()
                     .and_then(|result| result.ok())
                     .map(|byte| byte as u8)
                     .and_then(|u|  {
                         memory[pointer] = u;
                         Some(Interpreter {
                             memory,
                             pointer,
                             current_node: (*current_node).left,
                             while_stack
                         })

                 })
             }
             CommandType::Set => {
                 io::stdout().write(&[memory[pointer]]);
                 // println!("{:?}", memory[pointer]);
                 Some(Interpreter {
                     memory,
                     pointer,
                     current_node: (*current_node).left,
                     while_stack
                 })
             }
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
    let mut option_interpreter: Option<Interpreter> = parse(&mut v2).and_then(|ast|
        Some(Interpreter {
            memory: vec![0; 100],
            pointer: 0,
            current_node: Some(Box::new(ast)),
            while_stack: vec![],
        })
    );
    // println!("{:?}", io::stdin().bytes());

    while option_interpreter.is_some() {
        option_interpreter = option_interpreter.and_then(execute);
        // println!("{:?}", option_interpreter);
    }
    Ok(())
}