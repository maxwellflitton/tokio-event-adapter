

#[macro_export]
macro_rules! config_event_runtime {
    () => {
        pub mod tokio_event_adapter_runtime {

            use std::sync::LazyLock;
            use std::sync::Mutex;
            use std::sync::Arc;
            use std::collections::HashMap;
            use serde::{Serialize, Deserialize};
            use std::future::Future;
            use std::pin::Pin;

            pub type EventFunctionBuffer = Vec<EventFunction>;
            pub type EventFunction = fn(Vec<u8>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

            static HASHMAP: LazyLock<Arc<Mutex<HashMap<String, EventFunctionBuffer>>>> = LazyLock::new(|| {
                println!("initializing");
                Arc::new(Mutex::new(HashMap::new()))
            });

            pub fn insert_into_hashmap(name: String, func: EventFunction) -> () {
                let mut buffer = get_from_hashmap(&name).unwrap_or_else(|| vec![]);
                buffer.push(func);
                HASHMAP.lock().unwrap().insert(name, vec![func]);
            }

            pub fn get_from_hashmap(name: &str) -> Option<EventFunctionBuffer> {
                HASHMAP.lock().unwrap().get(name).cloned()
            }

            pub fn publish_event(name: &str, data: Vec<u8>) -> () {
                let buffer = match get_from_hashmap(name) {
                    Some(b) => b,
                    None => return
                };
                for f in buffer {
                    let boxed_future = f(data.clone());
                    tokio::spawn(async move {
                        boxed_future.await;
                    });
                }
            }

        }
    };
}
