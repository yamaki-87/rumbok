use crate::{
    consts::FAILED_TO_PARSE_MSG,
    utils::{self},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field};
pub trait AccessorGenerator {
    fn get_derive_id(&self) -> &str;

    fn crete_accessor_token_stream(
        &self,
        field_name: &syn::Ident,
        field_ty: &syn::Type,
    ) -> TokenStream;

    fn crete_accessor(&self, fields: &Field) -> Option<TokenStream> {
        let field_name = fields.ident.as_ref().unwrap();
        let field_ty = &fields.ty;
        let attrs = &fields.attrs;
        let parse_attr = match utils::parse_attr(attrs, &self.get_derive_id().to_lowercase()) {
            Ok(parse_attr) => parse_attr,
            Err(e) => {
                return Some(syn::Error::new_spanned(field_name, e.to_string()).to_compile_error());
            }
        };

        if parse_attr == utils::Attr::Skip {
            return None;
        }

        Some(self.crete_accessor_token_stream(field_name, field_ty))
    }

    fn create_ast(&self, input: TokenStream) -> TokenStream {
        let derive_input: DeriveInput = syn::parse2(input).expect(FAILED_TO_PARSE_MSG);
        let struct_name = &derive_input.ident;

        let fields = match utils::get_fields(struct_name, &derive_input.data, self.get_derive_id())
        {
            Ok(fields) => fields,
            Err(e) => {
                return e;
            }
        };
        let generics = &derive_input.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let accessor = fields.iter().map(|f| self.crete_accessor(f));

        let expanded = quote! {
            impl #impl_generics #struct_name #ty_generics #where_clause {
                #(#accessor)*
            }
        };

        expanded
    }
}
