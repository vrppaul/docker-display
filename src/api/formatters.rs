use colored::*;
use termion::{clear, cursor};

use super::structs::ContainersHashMap;

pub fn make_containers_table(containers: &mut ContainersHashMap) {
    // Clear the console and print the updated container list
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    println!(
        "┌─{}─┬─{}─┬─{}─┐",
        std::iter::repeat('─').take(12).collect::<String>(),
        std::iter::repeat('─').take(32).collect::<String>(),
        std::iter::repeat('─').take(32).collect::<String>(),
    );
    println!(
        "│ {}{} │ {}{} │ {}{} │",
        "ID".bold(),
        std::iter::repeat(' ').take(10).collect::<String>(),
        "Image".bold(),
        std::iter::repeat(' ').take(27).collect::<String>(),
        "Name".bold(),
        std::iter::repeat(' ').take(28).collect::<String>(),
    );
    println!(
        "└─{}─┴─{}─┴─{}─┘",
        std::iter::repeat('─').take(12).collect::<String>(),
        std::iter::repeat('─').take(32).collect::<String>(),
        std::iter::repeat('─').take(32).collect::<String>(),
    );

    containers
        .into_iter()
        .filter(|(_, pc)| !pc.is_empty())
        .for_each(|(project, project_containers)| {
            let is_empty = project_containers.is_empty();
            println!("┌{}┐", std::iter::repeat('─').take(84).collect::<String>());
            println!(
                "│ {:82} │",
                project
                    .as_ref()
                    .unwrap_or(&"uncategorized".to_string())
                    .blue()
                    .bold()
            );
            let table_element = if is_empty { "─" } else { "┬" };
            println!(
                "├─{}─{}─{}─{}─{}─┤",
                std::iter::repeat('─').take(12).collect::<String>(),
                table_element,
                std::iter::repeat('─').take(32).collect::<String>(),
                table_element,
                std::iter::repeat('─').take(32).collect::<String>(),
            );
            for container in project_containers.values() {
                println!(
                    "│ {} │ {:32} │ {:32} │",
                    container.id.chars().take(12).collect::<String>(),
                    container.image.chars().take(32).collect::<String>(),
                    container.name.chars().take(32).collect::<String>()
                );
            }
            println!(
                "└─{}─┴─{}─┴─{}─┘",
                std::iter::repeat('─').take(12).collect::<String>(),
                std::iter::repeat('─').take(32).collect::<String>(),
                std::iter::repeat('─').take(32).collect::<String>(),
            );
        });
}
