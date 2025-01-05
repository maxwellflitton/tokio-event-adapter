use tokio_event_adapter::{publish_event, subscribe_to_event};
use serde::{Serialize, Deserialize};
use crate::Two;


#[derive(Serialize, Deserialize, Debug)]
pub struct AddNumbers {
    pub num1: i32,
    pub num2: i32,
}


#[subscribe_to_event]
pub async fn add_numbers(add_numbers: AddNumbers) {
    println!("Adding numbers: {:?}", add_numbers);
    let result = add_numbers.num1 + add_numbers.num2;
    println!("Result: {}", result);
    let two = Two {};
    publish_event!(two);
}

