use std::{collections::HashMap, error::Error};

use serde_json::Value;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::unix::{OwnedReadHalf, OwnedWriteHalf},
};

use super::{errors::DockerAPIError, formatters::make_containers_table, structs::Container};

static HTTP_REQUEST: &str = "GET /containers/json HTTP/1.1\r\nHost: localhost\r\n\r\n";

pub async fn read_initial_docker_containers(
    reader: &mut BufReader<OwnedReadHalf>,
    writer: &mut OwnedWriteHalf,
) -> Result<HashMap<Option<String>, HashMap<String, Container>>, Box<dyn Error>> {
    writer.write_all(HTTP_REQUEST.as_bytes()).await?;

    let mut containers: HashMap<Option<String>, HashMap<String, Container>> = HashMap::new();
    containers.insert(None, HashMap::new());

    let mut lines = reader.lines();
    let mut body = String::new();
    while let Some(line) = lines.next_line().await? {
        if line.starts_with("[") {
            body = line;
            break;
        }
    }

    if body.is_empty() {
        return Err(DockerAPIError {
            api_type: "containers",
            message: "empty body",
        }
        .into());
    }

    let containers_json: Value = serde_json::from_str(&body)?;

    for container in containers_json.as_array().unwrap() {
        let (container, project) = parse_container_from_json(container);
        let project_containers: &mut HashMap<String, Container> =
            containers.entry(project).or_insert(HashMap::new());
        project_containers.insert(container.id.clone(), container);
    }

    make_containers_table(&mut containers);

    Ok(containers)
}

fn parse_container_from_json(container_json: &Value) -> (Container, Option<String>) {
    let id = container_json["Id"].as_str().unwrap_or("").to_string();
    let image = container_json["Image"].as_str().unwrap_or("").to_string();
    let name = container_json["Names"][0]
        .as_str()
        .unwrap_or("")
        .to_string()
        .split_off(1);
    let project = container_json["Labels"]["com.docker.compose.project"]
        .as_str()
        .map(|s| s.to_string());

    (Container { id, image, name }, project)
}
