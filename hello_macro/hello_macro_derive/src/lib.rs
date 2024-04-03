use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // parse the data of struct
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // get name struct
    let name = &ast.ident;
    // quote is used to replace #name by name of struct
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify used to convert the name in format code to String
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
