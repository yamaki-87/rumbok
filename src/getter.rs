use proc_macro2::TokenStream;
use quote::quote;

use crate::{accessor_generator::AccessorGenerator, utils};

pub const DERIVE_ID: &str = "Getter";
struct GenerateGetter;
impl GenerateGetter {
    pub fn new() -> Self {
        Self {}
    }
}
impl AccessorGenerator for GenerateGetter {
    fn get_derive_id(&self) -> &str {
        DERIVE_ID
    }

    fn crete_accessor_token_stream(
        &self,
        field_name: &syn::Ident,
        field_ty: &syn::Type,
        attr: utils::Attr,
    ) -> TokenStream {
        let getter_name = quote::format_ident!("get_{}", field_name);
        match attr {
            utils::Attr::Skip => TokenStream::new(),

            utils::Attr::Clone => {
                quote! {
                    pub fn #getter_name(&self) -> #field_ty
                    where
                        #field_ty: ::core::clone::Clone,
                    {
                        ::core::clone::Clone::clone(&self.#field_name)
                    }
                }
            }
            // 通常の &T / &mut T / T getter
            utils::Attr::Default => match field_ty {
                // フィールドが &T / &mut T の場合
                syn::Type::Reference(type_reference) => {
                    let inner_ty = &type_reference.elem;
                    quote! {
                        pub fn #getter_name(&self) -> &#inner_ty {
                            self.#field_name
                        }
                    }
                }
                // フィールドが T の場合
                _ => {
                    quote! {
                        pub fn #getter_name(&self) -> &#field_ty {
                            &self.#field_name
                        }
                    }
                }
            },
        }
    }
}

pub fn getter(input: TokenStream) -> TokenStream {
    let generator = GenerateGetter::new();
    generator.create_ast(input)
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
                #[d(a)]
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
    #[test]
    fn generates_getters_for_skip_field() {
        let input: TokenStream = quote! {
            struct Wrapper<'a,T>
            where
                T: Clone,
            {
                #[getter(skip)]
                value: &'a T,
            }
        };

        let output = getter(input);
        let output_str = output.to_string();
        eprintln!("{}", &output_str);
        assert!(!output_str.contains("pub fn get_value (& self) -> & T { & self . value }"));
    }
    #[test]
    fn generates_getters_for_skip_fields() {
        let input: TokenStream = quote! {
            struct User{
                id:i32,
                name:String,
                #[getter(skip)]
                age:Option<i32>,
            }
        };

        let output = getter(input);
        let output_str = output.to_string();

        eprintln!("{}", &output_str);
        assert!(output_str.contains("impl User"));
        assert!(output_str.contains("pub fn get_id (& self) -> & i32 { & self . id }"));
        assert!(output_str.contains("pub fn get_name (& self) -> & String { & self . name }"));
        assert!(
            !output_str.contains("pub fn get_age (& self) -> & Option < i32 > { & self . age }")
        );
    }
    #[test]
    fn expected_getters_err_msg() {
        let input: TokenStream = quote! {
            struct User{
                #[getter(aaaa)]
                age:Option<i32>,
            }
        };

        let output = getter(input);
        let output_str = output.to_string();
        eprintln!("{}", &output_str);
        assert!(output_str.contains("unknown getter option"));
    }
    #[test]
    fn expected_getters_clone() {
        let input: TokenStream = quote! {
            struct User{
                #[getter(clone)]
                age:Option<i32>,
            }
        };

        let output = getter(input);
        let output_str = output.to_string();
        eprintln!("{}", &output_str);
        assert!(
            output_str.contains("pub fn get_age (& self) -> Option < i32 > where Option < i32 > : :: core :: clone :: Clone , { :: core :: clone :: Clone :: clone (& self . age) }")
        );
    }
    #[test]
    fn expected_getters_with_skip_clone() {
        let input: TokenStream = quote! {
            struct User{
                #[getter(clone,skip)]
                age:Option<i32>,
            }
        };

        let output = getter(input);
        let output_str = output.to_string();
        eprintln!("{}", &output_str);
        assert!(
            !output_str.contains("pub fn get_age (& self) -> Option < i32 > where Option < i32 > : :: core :: clone :: Clone , { :: core :: clone :: Clone :: clone (& self . age) }")
        );
    }
}
