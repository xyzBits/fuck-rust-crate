pub trait Hello {
    fn hello(&self) -> String;
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let ans = quote! {
        impl Hello for #name {
            fn hello(&self) -> String {
                format!("Hello, I'm {}!", stringify!(#name))
            }
        }
    };

    ans.into()
}
