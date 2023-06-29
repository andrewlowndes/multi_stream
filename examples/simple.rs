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
            dbg!((a, b));
        })
        .await;
}
