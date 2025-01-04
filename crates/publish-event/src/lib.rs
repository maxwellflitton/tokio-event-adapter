extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, format_ident, ToTokens};
use syn::{parse_macro_input, ItemStruct, Ident, Item, DataStruct};


#[proc_macro]
pub fn publish_event(input: TokenStream) -> TokenStream {
    let instance_name = parse_macro_input!(input as Ident);

    let expanded = quote! {
        {
            let debug_display = format!("{:?}", #instance_name);
            let name = debug_display.split("{").nth(0).unwrap();
            let data = bincode::serialize(&#instance_name).unwrap();
            crate::tokio_event_adapter_runtime::publish_event(name, data);
        }
    };

    TokenStream::from(expanded)
}
