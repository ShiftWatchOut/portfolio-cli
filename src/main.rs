use std::{
    env, fs,
    io::{self, BufRead},
};

use chrono::prelude::*;
use console::style;
use dialoguer::{Input, MultiSelect};

enum PromptType {
    Input,
    Outter,
    Multiselect,
}

struct PromptConfig<'a> {
    name: &'a str,
    type_name: PromptType,
    message: &'a str,
    initial: &'a str,
}

fn run() -> io::Result<()> {
    let current_day = Local::now().naive_local().date().to_string();
    println!("今天是 {}", current_day);
    for arg in env::args() {
        println!("arg: {}", style(arg).green());
    }
    let source = fs::read_to_string("./template.md").unwrap();
    let configs = vec![
        // PromptConfig {
        //     name: "title",
        //     type_name: PromptType::Input,
        //     message: "博文标题",
        //     initial: "New Post",
        // },
        // PromptConfig {
        //     name: "filename",
        //     type_name: PromptType::Input,
        //     message: "MD 文件名(需要后缀)",
        //     initial: "new-post.md",
        // },
        // PromptConfig {
        //     name: "description",
        //     type_name: PromptType::Input,
        //     message: "简单说明",
        //     initial: "description text",
        // },
        PromptConfig {
            name: "date",
            type_name: PromptType::Outter,
            message: "来自外部的时间",
            initial: &current_day,
        },
        PromptConfig {
            name: "tag",
            type_name: PromptType::Multiselect,
            message: "标签分类",
            initial: "",
        },
    ];
    let mut new_content = source;
    for config in configs {
        let input: String;
        match config.type_name {
            PromptType::Input => {
                input = Input::<String>::new()
                    .with_prompt(config.message)
                    .with_initial_text(config.initial)
                    .interact_text()?;
            }
            PromptType::Multiselect => {
                let existing_tags = get_posts_tags("md_target");
                let chosen: Vec<usize> = MultiSelect::new().items(&existing_tags).interact()?;
                input = chosen
                    .iter()
                    .map(|i| existing_tags[*i].clone())
                    .collect::<Vec<_>>()
                    .join(", ");
            }
            PromptType::Outter => {
                input = String::from(config.initial);
            }
        }
        new_content = new_content.replace(
            ("{{ ".to_owned() + config.name + " }}").as_str(),
            input.as_str(),
        );
    }
    let target_file_name = "md_target/hello.md";
    fs::write(target_file_name, new_content)?;
    Ok(())
}

fn get_posts_tags(target_dir: &str) -> Vec<String> {
    let mut tags = vec![];
    let tag_start = "tag:";
    for entry in fs::read_dir(target_dir).unwrap() {
        let path = entry.unwrap().path();
        if !path.is_dir() {
            let file = fs::File::open(path).unwrap();
            let fin = io::BufReader::new(file).lines();
            for line in fin {
                let l = line.unwrap();
                if l.starts_with(tag_start) {
                    let tmp = l.replace(tag_start, "");
                    let tmp2 = tmp
                        .trim()
                        .split(", ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                    for t in tmp2.iter() {
                        if !tags.contains(t) {
                            tags.push(t.clone());
                        }
                    }
                    break;
                }
            }
        }
    }
    tags
}

fn main() {
    run().unwrap();
}
