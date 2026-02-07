use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field};

use crate::{consts::FAILED_TO_PARSE_NSG, utils::get_fields};

pub const DERIVE_ID: &str = "Getter";

fn getter_create(fields: &Field) -> TokenStream {
    let field_name = fields.ident.as_ref().unwrap();
    let field_ty = &fields.ty;

    let getter_name = quote::format_ident!("get_{}", field_name);
    match field_ty {
        // フィールドが &T / &mut T
        syn::Type::Reference(type_reference) => {
            let inner_ty = &type_reference.elem;
            quote! {
                pub fn #getter_name(&self) -> &#inner_ty {
                    self.#field_name
                }
            }
        }
        // フィールドが T
        _ => {
            quote! {
                    pub fn #getter_name(&self) -> &#field_ty {
                        &self.#field_name
                    }
            }
        }
    }
}

pub fn getter(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse2(input).expect(FAILED_TO_PARSE_NSG);
    let struct_name = &derive_input.ident;

    let fields = match get_fields(struct_name, &derive_input.data, DERIVE_ID) {
        Ok(fields) => fields,
        Err(e) => {
            return e;
        }
    };
    let generics = &derive_input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let getters = fields.iter().map(|f| getter_create(f));

    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#getters)*
        }
    };

    expanded
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generates_getters_for_named_fields() {
        let input: TokenStream = quote! {
            struct User{
                id:i32,
                name:String,
                age:Option<i32>,
            }
        };

        let output = getter(input);
        let output_str = output.to_string();
        eprintln!("{}", &output_str);
        // ざっくりパターンを見る（厳密パースじゃなくてもまずはOK）
        assert!(output_str.contains("impl User"));
        assert!(output_str.contains("pub fn get_id (& self) -> & i32 { & self . id }"));
        assert!(output_str.contains("pub fn get_name (& self) -> & String { & self . name }"));
        assert!(
            output_str.contains("pub fn get_age (& self) -> & Option < i32 > { & self . age }")
        );
    }
    #[test]
    fn error_on_unnamed_fields() {
        let input: TokenStream = quote! {
            struct Tuple(i32);
        };

        let output = getter(input);
        let output_str = output.to_string();

        assert!(output_str.contains("Getter only supports structs with named fields"));
    }
    #[test]
    fn error_on_non_struct() {
        let input: TokenStream = quote! {
            enum E {
                A,
                B,
            }
        };

        let output = getter(input);
        let output_str = output.to_string();

        assert!(output_str.contains("Getter can only be derived for structs"));
    }

    #[test]
    fn supports_generics() {
        let input: TokenStream = quote! {
            struct Wrapper<T>
            where
                T: Clone,
            {
                value: T,
            }
        };

        let output = getter(input);
        let output_str = output.to_string();

        // impl <T> Wrapper <T> where T: Clone { ... }
        assert!(output_str.contains("impl < T > Wrapper < T > where T : Clone"));
        assert!(output_str.contains("pub fn get_value (& self) -> & T { & self . value }"));
    }

    #[test]
    fn supports_refer() {
        let input: TokenStream = quote! {
            struct Wrapper<'a>
            {
                value: &'a str,
            }
        };

        let output = getter(input);
        let output_str = output.to_string();
        eprintln!("{}", &output_str);
        assert!(output_str.contains("pub fn get_value (& self) -> & str { self . value }"));
    }
}
