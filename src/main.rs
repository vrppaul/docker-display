pub mod api;
pub mod client;

use std::error::Error;

use crate::api::events;


#[tokio::main]
async fn main() {
    launch_display().await.ok();
}

async fn launch_display() -> Result<(), Box<dyn Error>> {
    
    let (reader, writer) = client::get_stream_tuple().await?;

    events::read_docker_events(reader, writer).await?;

    Ok(())
}
