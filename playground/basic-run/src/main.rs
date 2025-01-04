use tokio_event_adapter::subscribe_to_event;
use tokio_event_adapter::config_event_runtime;
use tokio_event_adapter::publish_event;

use serde::{Serialize, Deserialize};

config_event_runtime!();

#[derive(Serialize, Deserialize, Debug)]
struct One;

#[derive(Serialize, Deserialize, Debug)]
struct Two;

#[subscribe_to_event]
async fn test(one: One) {
    println!("calling from test function with: {:?}", one);
    let two = Two {};
    publish_event!(two);
}

#[subscribe_to_event]
async fn test2(two: Two) {
    println!("calling from test2 function with: {:?}", two);
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let one = One {};
    test(one).await;
    let two = One {};
    publish_event!(two);
    std::thread::sleep(std::time::Duration::from_secs(5));
}

