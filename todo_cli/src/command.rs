pub enum Command {
    Add(String),
    List,
    Done(u32),
    Remove(u32),
    Show(u32),
    Edit(u32, String),
    Invalid,
    Exit,
}

pub fn parse_command(args: Vec<String>) -> Command {
    if args.is_empty() {
        return Command::Invalid;
    }

    match args[0].as_str() {
        "add" => {
            if args.len() < 2 {
                Command::Invalid
            } else {
                let content = args[1..].join(" ");
                if content.trim().is_empty() {
                    Command::Invalid
                } else {
                    Command::Add(content)
                }
            }
        }
        "list" => Command::List,
        "done" => {
            if args.len() < 2 {
                Command::Invalid
            } else {
                match args[1].parse::<u32>() {
                    Ok(id) => Command::Done(id),
                    Err(_) => Command::Invalid,
                }
            }
        }
        "remove" => {
            if args.len() < 2 {
                Command::Invalid
            } else {
                match args[1].parse::<u32>() {
                    Ok(id) => Command::Remove(id),
                    Err(_) => Command::Invalid,
                }
            }
        }
        "show" => {
            if args.len() < 2 {
                Command::Invalid
            } else {
                match args[1].parse::<u32>() {
                    Ok(id) => Command::Show(id),
                    Err(_) => Command::Invalid,
                }
            }
        }
        "edit" => {
            if args.len() < 3 {
                Command::Invalid
            } else {
                match args[1].parse::<u32>() {
                    Ok(id) => {
                        let content = args[2..].join(" ");
                        if content.trim().is_empty() {
                            Command::Invalid
                        } else {
                            Command::Edit(id, content)
                        }
                    }
                    Err(_) => Command::Invalid,
                }
            }
        }
        "exit" => Command::Exit,
        _ => Command::Invalid,
    }
}

pub fn tokenize_input(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '"' => {
                in_quote = !in_quote;
            }
            ' ' | '\t' | '\n' | '\r' => {
                if in_quote {
                    current.push(c);
                } else if !current.is_empty() {
                    tokens.push(current);
                    current = String::new();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}