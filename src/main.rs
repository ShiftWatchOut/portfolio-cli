use std::{env, fs, io};

use console::style;
use dialoguer::Input;

struct TextConfig<'a> {
    name: &'a str,
    type_name: &'a str,
    message: &'a str,
    initial: &'a str,
}

fn run() -> io::Result<()> {
    for arg in env::args() {
        println!("arg: {}", style(arg).green());
    }
    let source = fs::read_to_string("./template.md").unwrap();
    let first_config = TextConfig {
        name: "title",
        type_name: "text",
        message: "博文标题",
        initial: "New Post",
    };
    let input = Input::<String>::new()
        .with_prompt(first_config.message)
        .with_initial_text(first_config.initial)
        .interact_text()?;
    println!("prompt result: {}", input);
    let target_file_name = "hello.md";
    let new_content = source.as_str();
    let new_content = new_content.replace(
        ("{{ ".to_owned() + first_config.name + " }}").as_str(),
        input.as_str(),
    );
    // println!("doc: \n{}", new_content);
    fs::write(target_file_name, new_content)?;
    Ok(())
}

fn main() {
    run().unwrap();
}
