use anyhow::Result;
use clap::Parser;
use sea_streamer_iggy::IggyStreamer;
use sea_streamer_types::{Producer, StreamUrl, Streamer};

#[derive(Debug, Parser)]
struct Args {
    #[clap(
        long,
        help = "Streamer URI with stream key, i.e. try `iggy://localhost:8090/hello`",
        env = "STREAM_URL"
    )]
    stream: StreamUrl,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let Args { stream } = Args::parse();

    let streamer = IggyStreamer::connect(stream.streamer(), Default::default()).await?;
    let producer = streamer
        .create_producer(stream.stream_key()?, Default::default())
        .await?;

    for i in 0..1000 {
        let message = format!("{{\"hello\": {i}}}");
        producer.send(message)?;
    }

    streamer.disconnect().await?;

    Ok(())
}
