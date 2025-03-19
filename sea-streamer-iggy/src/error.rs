pub use iggy::error::IggyError as IggyErr;
use sea_streamer_types::{StreamErr, StreamResult};

pub type IggyResult<T> = StreamResult<T, IggyErr>;

pub(crate) fn stream_err(err: IggyErr) -> StreamErr<IggyErr> {
    StreamErr::Backend(err)
}
