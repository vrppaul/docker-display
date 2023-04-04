use std::collections::HashMap;
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
use std::error::Error;
use termion::{clear, cursor};
use colored::*;

static HTTP_REQUEST: &str = "GET /events HTTP/1.1\r\nHost: localhost\r\n\r\n";

#[derive(Debug)]
struct Container {
    id: String,
    image: String,
    name: String,
    status: String,
}

pub async fn read_docker_events(mut reader: BufReader<OwnedReadHalf>, mut writer: OwnedWriteHalf) -> Result<(), Box<dyn Error>> {
    writer.write_all(HTTP_REQUEST.as_bytes()).await?;

    let mut line = String::new();
    let mut containers: HashMap<String, Container> = HashMap::new();

    println!("Starting reading docker events");

    while reader.read_line(&mut line).await? > 0 {
        if line.starts_with('{') {
            let event: Value = serde_json::from_str(&line)?;
            println!("{}", event);
            let event_type = event["Type"].as_str().unwrap_or("");
            let status = event["status"].as_str().unwrap_or("");
            let image = event["Actor"]["Attributes"]["image"].as_str().unwrap_or("");
            let name = event["Actor"]["Attributes"]["name"].as_str().unwrap_or("");

            if event_type == "container" && (status == "start" || status == "die") {
                let container_id = event["id"].as_str().unwrap_or("").to_string();
                let display_status = if status == "start" { "alive" } else { "dead" };
                let container = Container {
                    id: container_id.clone(),
                    image: image.to_string(),
                    name: name.to_string(),
                    status: display_status.to_string(),
                };

                if status == "start" {
                    containers.insert(container_id, container);
                } else if status == "die" {
                    if let Some(dead_container) = containers.get_mut(&container_id) {
                        dead_container.status = "dead".to_string();
                    }
                }

                // Clear the console and print the updated container list
                print!("{}{}", clear::All, cursor::Goto(1, 1));
                println!(
                    "┌ {} ┬ {} ┬ {} ┬ {} ┐",
                    std::iter::repeat('─').take(12).collect::<String>(),
                    std::iter::repeat('─').take(32).collect::<String>(),
                    std::iter::repeat('─').take(32).collect::<String>(),
                    std::iter::repeat('─').take(6).collect::<String>(),
                );
                println!(
                    "│ ID{} │ Image{} │ Name{} │ Status │",
                    std::iter::repeat(' ').take(10).collect::<String>(),
                    std::iter::repeat(' ').take(27).collect::<String>(),
                    std::iter::repeat(' ').take(28).collect::<String>(),
                );
                println!(
                    "├ {} ┼ {} ┼ {} ┼ {} ┤",
                    std::iter::repeat('─').take(12).collect::<String>(),
                    std::iter::repeat('─').take(32).collect::<String>(),
                    std::iter::repeat('─').take(32).collect::<String>(),
                    std::iter::repeat('─').take(6).collect::<String>(),
                );

                for container in containers.values() {
                    let status_colored = if container.status == "alive" {
                        container.status.green()
                    } else {
                        container.status.red()
                    };
                    println!(
                        "│ {} │ {:32} │ {:32} │ {:6} │",
                        container.id.chars().take(12).collect::<String>(),
                        container.image.chars().take(32).collect::<String>(),
                        container.name.chars().take(32).collect::<String>(),
                        status_colored
                    );
                }
                println!(
                    "└ {} ┴ {} ┴ {} ┴ {} ┘",
                    std::iter::repeat('─').take(12).collect::<String>(),
                    std::iter::repeat('─').take(32).collect::<String>(),
                    std::iter::repeat('─').take(32).collect::<String>(),
                    std::iter::repeat('─').take(6).collect::<String>(),
                );
            }
        }
        line.clear();
    }

    Ok(())
}
