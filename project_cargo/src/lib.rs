use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
// like a proxy or a decorator in typescript
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    println!("{} defined", input.sig.ident);
    println!("Args received: {}", attr.to_string());
    let function_name = &input.sig.ident;
    let function_body = &input.block;
    let output = quote! {
        #[allow(non_snake_case)]
        fn #function_name() {
            println!("before macro");
            #function_body
            println!("after macro");
        }
    };

    output.into()
}
