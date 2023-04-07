use std::collections::HashMap;
use std::error::Error;

use colored::*;
use serde_json::Value;
use termion::{clear, cursor};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::unix::{OwnedReadHalf, OwnedWriteHalf},
};

use crate::api::formatters::make_containers_table;

use super::structs::{Container, ContainersHashMap};

// static HTTP_REQUEST: &str = create_api_url("/events");
static HTTP_REQUEST: &str = "GET /events HTTP/1.1\r\nHost: localhost\r\n\r\n";

pub async fn read_docker_events(
    reader: &mut BufReader<OwnedReadHalf>,
    writer: &mut OwnedWriteHalf,
    containers: &mut ContainersHashMap,
) -> Result<(), Box<dyn Error>> {
    writer.write_all(HTTP_REQUEST.as_bytes()).await?;

    let mut line = String::new();

    while reader.read_line(&mut line).await? > 0 {
        // We don't want to get the HTTP meta information
        if line.starts_with('{') {
            let event: Value = serde_json::from_str(&line)?;
            let event_type = event["Type"].as_str().unwrap_or("");
            let status = event["status"].as_str().unwrap_or("");
            let image = event["Actor"]["Attributes"]["image"].as_str().unwrap_or("");
            let name = event["Actor"]["Attributes"]["name"].as_str().unwrap_or("");

            if event_type == "container" && (status == "start" || status == "die") {
                let container_id = event["id"].as_str().unwrap_or("").to_string();
                let container = Container {
                    id: container_id.clone(),
                    image: image.to_string(),
                    name: name.to_string(),
                };

                let project = event["Actor"]["Attributes"]["com.docker.compose.project"]
                    .as_str()
                    .map(|s| s.to_string());
                if status == "start" {
                    let project_containers: &mut HashMap<String, Container> =
                        containers.entry(project).or_insert(HashMap::new());
                    project_containers.insert(container_id, container);
                } else if status == "die" {
                    if let Some(project_containers) = containers.get_mut(&project) {
                        project_containers.remove(&container_id);
                    }
                }

                make_containers_table(containers);
            }
        }
        line.clear();
    }

    Ok(())
}
