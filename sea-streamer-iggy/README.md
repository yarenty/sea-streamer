### `sea-streamer-iggy`: Iggy Backend

This is the Iggy backend implementation for SeaStreamer.
This crate provides a comprehensive type system that makes working with Iggy easier and safer.

First of all, all API (many are sync) are properly wrapped as async. Methods are also marked `&mut` to eliminate possible race conditions.

`IggyConsumerOptions` has typed parameters.

`IggyConsumer` allows you to `seek` to point in time, `rewind` to particular offset, and `commit` message read.

`IggyProducer` allows you to `await` a send `Receipt` or discard it if you are uninterested. You can also flush the Producer.

`Iggytreamer` allows you to flush all producers on `disconnect`.

See [tests](https://github.com/SeaQL/sea-streamer/blob/main/sea-streamer-iggy/tests/consumer.rs) for an illustration of the stream semantics.

This crate depends on [`iggy`](https://docs.iggy.rs/),


SDK Reference: <https://docs.iggy.rs/introduction/high-level-sdk>
