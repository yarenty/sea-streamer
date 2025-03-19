use anyhow::Result;
use clap::Parser;
use sea_streamer_iggy::{AutoOffsetReset, IggyConsumerOptions, IggyStreamer};
use sea_streamer_types::{
    Buffer, Consumer, ConsumerMode, ConsumerOptions, Message, StreamUrl, Streamer,
};

#[derive(Debug, Parser)]
struct Args {
    #[clap(
        long,
        help = "Streamer URI with stream key(s), i.e. try `iggy://localhost:8090/hello`",
        env = "STREAM_URL"
    )]
    stream: StreamUrl,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let Args { stream } = Args::parse();

    let streamer = IggyStreamer::connect(stream.streamer(), Default::default()).await?;
    let mut options = IggyConsumerOptions::new(ConsumerMode::RealTime);
    options.set_auto_offset_reset(AutoOffsetReset::Earliest);
    let consumer = streamer
        .create_consumer(stream.stream_keys(), options)
        .await?;

    loop {
        let mess = consumer.next().await?;
        println!("{}", mess.message().as_str()?);
    }
}
