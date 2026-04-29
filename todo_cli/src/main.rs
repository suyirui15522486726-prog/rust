mod command;
mod storage;
mod todo;

use command::{parse_command, tokenize_input};
use storage::TodoStorage;
use std::io::{self, Write};

fn main() {
    let mut storage = TodoStorage::new();
    let mut input = String::new();

    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("读取失败");

        let tokens = tokenize_input(&input);
        let cmd = parse_command(tokens);

        match cmd {
            command::Command::Add(content) => {
                let id = storage.add(content);
                println!("Added todo #{}", id);
            }
            command::Command::List => {
                storage.list();
            }
            command::Command::Done(id) => {
                if storage.done(id) {
                    println!("ok");
                } else {
                    println!("错误：任务ID不存在");
                }
            }
            command::Command::Remove(id) => {
                if storage.remove(id) {
                    println!("ok");
                } else {
                    println!("错误：任务ID不存在");
                }
            }
            command::Command::Show(id) => {
                if !storage.show(id) {
                    println!("错误：任务ID不存在");
                }
            }
            command::Command::Edit(id, content) => {
                if storage.edit(id, content) {
                    println!("ok");
                } else {
                    println!("错误：任务ID不存在");
                }
            }
            command::Command::Exit => {
                break;
            }
            command::Command::Invalid => {
                println!("无效命令");
            }
        }
    }
}