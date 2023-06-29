# Multi stream
Aggregate multiple streams of different types in a single stream with an item type that is a tuple of the incoming stream items.

## Important
- The streams are polled and a new stream item is emitted if any if ANY of the streams emits a new value
- The values are cloned when the new stream value is emitted so incoming stream items must be clonable
- Items in the tuple being emitted are all returned in `Option` as their corresponding stream may never emit
- The macro supports up to 12 input streams only

## Example usage
- Add to your Cargo.toml file
```toml
[dependencies]
multi_stream = "0.1.0"
```
- Import the `muti_stream` macro and pass in stream instances to create a new stream:
```rust
use async_stream::stream;
use futures::StreamExt;
use multi_stream::multi_stream;
use rand::random;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let a = stream! {
        sleep(Duration::from_millis(random::<u8>().into())).await;
        yield 1;
        sleep(Duration::from_millis(random::<u8>().into())).await;
        yield 2;
        sleep(Duration::from_millis(random::<u8>().into())).await;
        yield 3;
    };
    let b = stream! {
        sleep(Duration::from_millis(random::<u8>().into())).await;
        yield 4;
        sleep(Duration::from_millis(random::<u8>().into())).await;
        yield 5;
        sleep(Duration::from_millis(random::<u8>().into())).await;
        yield 6;
    };

    multi_stream!(a, b)
        .for_each(|(a, b)| async move {
            //emitted when any of the streams has a new value
            dbg!((a, b));
        })
        .await;
}
```
