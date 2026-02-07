use proc_macro2::TokenStream;
use syn::{Data, Field, Ident, punctuated::Punctuated, token::Comma};

pub fn get_fields<'a>(
    ident: &'a Ident,
    data: &'a Data,
    derive_id: &str,
) -> Result<&'a Punctuated<Field, Comma>, TokenStream> {
    let fields = match data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_name) => &fields_name.named,
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    format!("{} only supports structs with named fields", derive_id),
                )
                .to_compile_error());
            }
        },
        _other => {
            return Err(syn::Error::new_spanned(
                ident,
                format!("{} can only be derived for structs", derive_id),
            )
            .to_compile_error());
        }
    };

    Ok(fields)
}
