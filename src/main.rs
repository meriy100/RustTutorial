use std::fs::File;
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
enum CommandType {
    Inc, Dec, PInc, PDec, Get, Set, While, End
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

fn main() -> io::Result<()> {
    let mut file = File::open("test.bf")?;
    let mut contents = String::new();
    let mut memory: Vec<u8> = vec![0, 100];

    file.read_to_string(&mut contents)?;
    let v2: Vec<CommandType> = contents.chars().filter_map(char_to_command_type).collect::<Vec<CommandType>>();
    println!("{:?}", v2);
    Ok(())
}