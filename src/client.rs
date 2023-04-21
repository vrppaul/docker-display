use std::error::Error;
use tokio::io::BufReader;
use tokio::net::unix::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::UnixStream;

pub async fn get_stream_tuple() -> Result<(BufReader<OwnedReadHalf>, OwnedWriteHalf), Box<dyn Error>>
{
    let stream = UnixStream::connect("/var/run/docker.sock").await?;
    let (reader, writer) = stream.into_split();

    Ok((BufReader::new(reader), writer))
}
