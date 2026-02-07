use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Ident, punctuated::Punctuated, token::Comma};

fn get_fields<'a>(
    ident: &'a Ident,
    data: &'a Data,
) -> Result<&'a Punctuated<Field, Comma>, TokenStream> {
    let fields = match data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_name) => &fields_name.named,
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "Setter only supports structs with named fields",
                )
                .to_compile_error()
                .into());
            }
        },
        _other => {
            return Err(
                syn::Error::new_spanned(ident, "Setter can only be derived for structs")
                    .to_compile_error()
                    .into(),
            );
        }
    };

    Ok(fields)
}

fn setter_create(fields: &Field) -> TokenStream {
    let field_name = fields.ident.as_ref().unwrap();
    let field_ty = &fields.ty;

    let setter_name = quote::format_ident!("set_{}", field_name);
    quote! {
        pub fn #setter_name(&mut self, value: #field_ty) {
            self.#field_name = value;
        }
    }
}

pub fn setter(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse2(input).expect("failed to parse input");
    let struct_name = &derive_input.ident;

    let fields = match get_fields(struct_name, &derive_input.data) {
        Ok(fields) => fields,
        Err(e) => {
            return e;
        }
    };
    let generics = &derive_input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let setters = fields.iter().map(|f| setter_create(f));

    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#setters)*
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generates_setters_for_named_fields() {
        let input: TokenStream = quote! {
            struct User{
                id:i32,
                name:String
            }
        };

        let output = setter(input);
        let output_str = output.to_string();
        // ざっくりパターンを見る（厳密パースじゃなくてもまずはOK）
        assert!(output_str.contains("impl User"));
        assert!(output_str.contains("fn set_id"));
        assert!(output_str.contains("fn set_name"));
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

        // impl <T> Wrapper <T> where T: Clone { ... }
        assert!(output_str.contains("impl < T > Wrapper < T > where T : Clone"));
        assert!(output_str.contains("fn set_value"));
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
        eprintln!("{}", output_str);
    }
}
