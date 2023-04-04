pub mod api;
pub mod client;

use std::error::Error;

use crate::client::get_stream_tuple;
use crate::api::events::read_docker_events;


#[tokio::main]
async fn main() {
    launch_display().await;
}

async fn launch_display() -> Result<(), Box<dyn Error>> {
    
    let (reader, writer) = get_stream_tuple().await?;

    read_docker_events(reader, writer).await?;

    Ok(())
}
