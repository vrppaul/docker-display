use tokio::net::UnixStream;
use tokio::io::BufReader;
use std::error::Error;

#[tokio::main]
pub async fn get_stream_tuple() -> Result<(), Box<dyn Error>> {
    let stream = UnixStream::connect("/var/run/docker.sock").await?;
    let (reader, mut writer) = stream.into_split();

    Ok((BufReader::new(reader), writer))
}