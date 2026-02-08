use proc_macro2::TokenStream;
use quote::quote;

use crate::{accessor_generator::AccessorGenerator, utils};

pub const DERIVE_ID: &str = "Setter";
struct GenerateSetter;
impl GenerateSetter {
    pub fn new() -> Self {
        Self {}
    }
}

impl AccessorGenerator for GenerateSetter {
    fn get_derive_id(&self) -> &str {
        DERIVE_ID
    }

    fn crete_accessor_token_stream(
        &self,
        field_name: &syn::Ident,
        field_ty: &syn::Type,
        _attr: utils::Attr,
    ) -> TokenStream {
        let setter_name = quote::format_ident!("set_{}", field_name);
        quote! {
            pub fn #setter_name(&mut self, #field_name: #field_ty) {
                self.#field_name = #field_name;
            }
        }
    }
}

pub fn setter(input: TokenStream) -> TokenStream {
    let generator = GenerateSetter::new();
    generator.create_ast(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generates_setters_for_named_fields() {
        let input: TokenStream = quote! {
            struct User{
                id:i32,
                name:String,
                #[d(a)]
                err:Result<String,String>
            }
        };

        let output = setter(input);
        let output_str = output.to_string();
        eprintln!("{}", &output_str);

        assert!(output_str.contains("impl User"));
        assert!(output_str.contains("pub fn set_id (& mut self , id : i32) { self . id = id ; } "));
        assert!(
            output_str
                .contains("pub fn set_name (& mut self , name : String) { self . name = name ; }")
        );
        assert!(output_str.contains(
            "pub fn set_err (& mut self , err : Result < String , String >) { self . err = err ; }"
        ));
    }

    #[test]
    fn error_on_unnamed_fields() {
        let input: TokenStream = quote! {
            struct Tuple(i32);
        };

        let output = setter(input);
        let output_str = output.to_string();

        assert!(output_str.contains("Setter only supports structs with named fields"));
    }
    #[test]
    fn error_on_non_struct() {
        let input: TokenStream = quote! {
            enum E {
                A,
                B,
            }
        };

        let output = setter(input);
        let output_str = output.to_string();

        assert!(output_str.contains("Setter can only be derived for structs"));
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

        let output = setter(input);
        let output_str = output.to_string();

        eprintln!("{}", &output_str);
        // impl <T> Wrapper <T> where T: Clone { ... }
        assert!(output_str.contains("impl < T > Wrapper < T > where T : Clone"));
        assert!(
            output_str
                .contains("pub fn set_value (& mut self , value : T) { self . value = value ; }")
        );
    }
    #[test]
    fn supports_refer() {
        let input: TokenStream = quote! {
            struct Wrapper<'a>
            {
                value: &'a str,
            }
        };

        let output = setter(input);
        let output_str = output.to_string();
        eprintln!("{}", &output_str);
        assert!(output_str.contains(
            "pub fn set_value (& mut self , value : & 'a str) { self . value = value ; }"
        ));
    }
    #[test]
    fn generates_setters_for_skip_field() {
        let input: TokenStream = quote! {
            struct Wrapper<'a,T>
            where
                T: Clone,
            {
                #[setter(skip)]
                value: &'a T,
            }
        };

        let output = setter(input);
        let output_str = output.to_string();
        eprintln!("{}", &output_str);
        assert!(
            !output_str.contains(
                "pub fn set_value (& mut self , value : & 'a T) { self . value = value ; }"
            )
        );
    }
    #[test]
    fn generates_setters_for_skip_fields() {
        let input: TokenStream = quote! {
            struct User{
                id:i32,
                name:String,
                #[setter(skip)]
                age:Option<i32>,
            }
        };

        let output = setter(input);
        let output_str = output.to_string();

        eprintln!("{}", &output_str);
        assert!(output_str.contains("impl User"));
        assert!(output_str.contains("pub fn set_id (& mut self , id : i32) { self . id = id ; } "));
        assert!(
            output_str
                .contains("pub fn set_name (& mut self , name : String) { self . name = name ; }")
        );
        assert!(
            !output_str.contains(
                "pub fn set_age (& mut self , age : Option < i32 >) { self . age = age ; }"
            )
        );
    }
    #[test]
    fn expected_setters_err_msg() {
        let input: TokenStream = quote! {
            struct User{
                #[setter(aaaa)]
                age:Option<i32>,
            }
        };

        let output = setter(input);
        let output_str = output.to_string();
        eprintln!("{}", &output_str);
        assert!(output_str.contains("unknown getter option"));
    }
}
