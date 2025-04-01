use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};  // Add DeriveInput import

#[proc_macro_derive(CustomDebug)]
pub fn custom_debug_derive(input: TokenStream) -> TokenStream {
    // Parse the input struct with explicit type annotation
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Generate a Debug implementation
    impl_custom_debug(&ast)
}

fn impl_custom_debug(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated_code = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{} {{ Custom Debug! }}", stringify!(#name))
            }
        }
    };
    generated_code.into()
}

/*
// Add this to proc_macros/lib.rs along with the existing CustomDebug
#[proc_macro_derive(custom_derive_macro)]
pub fn custom_derive_macro(input: TokenStream) -> TokenStream {
    // Parse the input struct
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    
    // Generate some simple implementation
    let name = &ast.ident;
    let generated = quote! {
        impl #name {
            pub fn generated_method() {
                println!("Generated method for {}", stringify!(#name));
            }
        }
    };
    generated.into()
}
*/