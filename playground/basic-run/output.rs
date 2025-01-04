#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use tokio_event_adapter::subscribe_to_event;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::future::Future;
use std::pin::Pin;
struct One;
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for One {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_unit_struct(__serializer, "One")
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for One {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<One>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = One;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "unit struct One",
                    )
                }
                #[inline]
                fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    _serde::__private::Ok(One)
                }
            }
            _serde::Deserializer::deserialize_unit_struct(
                __deserializer,
                "One",
                __Visitor {
                    marker: _serde::__private::PhantomData::<One>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for One {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "One")
    }
}
async fn test(one: One) {
    {
        ::std::io::_print(format_args!("calling from test function with: {0:?}\n", one));
    };
}
#[doc(hidden)]
fn _check_test<T>()
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{}
#[doc(hidden)]
const _: fn() = || {
    _check_test::<One>();
};
mod register_mod_test {
    use crate::HASHMAP;
}
#[doc(hidden)]
fn routed_test(data: Vec<u8>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    std::boxed::Box::pin(async move {
        let deserialized: One = bincode::deserialize(&data).unwrap();
        test(deserialized).await;
    })
}
#[doc(hidden)]
fn register_test() {
    {
        ::std::io::_print(format_args!("Registering function: {0}\n", "\"One\""));
    };
    crate::HASHMAP
        .lock()
        .unwrap()
        .insert(
            "One".to_string(),
            <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([routed_test])),
        );
}
extern fn init_test() {
    {
        ::std::io::_print(format_args!("Initializing function: {0}\n", "test"));
    };
    register_test();
}
#[used]
#[allow(non_upper_case_globals, non_snake_case)]
#[doc(hidden)]
#[link_section = "__DATA,__mod_init_func"]
static init_test___rust_ctor___ctor: unsafe extern "C" fn() -> usize = {
    #[allow(non_snake_case)]
    unsafe extern "C" fn init_test___rust_ctor___ctor() -> usize {
        init_test();
        0
    }
    init_test___rust_ctor___ctor
};
static HASHMAP: LazyLock<
    Arc<
        Mutex<
            HashMap<
                String,
                Vec<
                    fn(
                        Vec<u8>,
                    ) -> Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>,
                >,
            >,
        >,
    >,
> = LazyLock::new(|| {
    {
        ::std::io::_print(format_args!("initializing\n"));
    };
    Arc::new(Mutex::new(HashMap::new()))
});
fn main() {
    let body = async {
        {
            ::std::io::_print(format_args!("Hello, world!\n"));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", HASHMAP));
        };
        let one = One;
        test(one).await;
        let two = One;
        let binding = HASHMAP.lock().unwrap();
        let func = binding.get("One").unwrap();
        for f in func {
            let boxed_future = f(bincode::serialize(&two).unwrap());
            tokio::spawn(async move {
                boxed_future.await;
            });
        }
        std::thread::sleep(std::time::Duration::from_secs(2));
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
