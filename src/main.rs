#![allow(dead_code)]
use futures::stream::{self, StreamExt};
use tokio::time::delay_for;
use std::time::Duration;
use serde_json::{Value, json};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let fun = lambda::handler_fn(handler);
    lambda::run(fun).await?;
    Ok(())
}

async fn handler(_event: Value) -> Result<Value, String> {
    let result = invoke_juniper();
    // If we don't use juniper and .await the futures normally, it works fine.
    // let result = test1(100).await;
    println!("result={:?}", result);
    Ok(json!({}))
}

// Juniper (https://crates.io/crates/juniper) is a synchronous library.
fn invoke_juniper() -> Vec<usize> {
    // And from it, we call into our async code using block_on
    futures::executor::block_on(test1(100))
}


// All three of these functions fail after executing ~60 futures.
async fn test1(n: usize) -> Vec<usize> {
    stream::iter(0..n).map(pause_and_return).buffer_unordered(10).collect().await
}

async fn test2(n: usize) -> Vec<usize> {
    stream::iter(0..n).map(pause_and_return).buffered(10).collect().await
}

async fn test3(n: usize) -> Vec<usize> {
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(pause_and_return(i).await);
    }
    vec
}

async fn pause_and_return(n: usize) -> usize {
    println!("Starting {}", n);
    delay_for(Duration::from_millis(100)).await;
    println!("Finished {}", n);
    n
}
