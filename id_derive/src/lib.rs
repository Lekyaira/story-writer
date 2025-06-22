pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HasId)]
pub fn derive_has_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let gen = quote! {
        impl HasId for #name {
            fn id(&self) -> &str {
                &self.id
            }
            fn generate_id(&self) -> String {
                crate::id::generate_struct_id(self)
            }
        }
        impl #name {
            pub fn new_with_id(mut self) -> Self {
                self.id = crate::id::generate_struct_id(&self);
                self
            }
        }
    };
    gen.into()
}
