extern crate proc_macro;
use proc_macro::TokenStream;
use quote::__private::Span;
use quote::quote;
use syn::parse_macro_input;
use syn::Ident;
use syn::ItemStruct;

#[proc_macro_derive(FlaawnComponentMacro)]
pub fn flaawn_component_macro(input: TokenStream) -> TokenStream {
    let s2 = parse_macro_input!(input as ItemStruct);
    let mut containsAtributtes = false;
    for field in s2.fields {
        if field.ident.unwrap().to_string() == "attributes" {
            containsAtributtes = true;
        }
        //println!("Second: {:?}", field.ident.unwrap().to_string());
    }
    let func = quote!(
        fn hello() {}
    );
    let r = quote! {
        #func
        fn hello2() {}
    };
    r.into()
}
