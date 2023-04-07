pub mod api;
pub mod client;

use std::error::Error;

use crate::api::{containers, events};

#[tokio::main]
async fn main() {
    launch_display().await.ok();
}

async fn launch_display() -> Result<(), Box<dyn Error>> {
    let (mut reader, mut writer) = client::get_stream_tuple().await?;

    let mut containers = containers::read_initial_docker_containers(&mut reader, &mut writer)
        .await
        .unwrap();
    events::read_docker_events(&mut reader, &mut writer, &mut containers).await?;

    Ok(())
}
