use proc_macro2::TokenStream;
use syn::{Data, Field, Ident, punctuated::Punctuated, token::Comma};

use crate::consts;

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
#[derive(Debug, PartialEq, Eq)]
pub enum Attr {
    Default,
    Skip,
    Clone,
}

pub fn parse_attr(attrs: &Vec<syn::Attribute>, derive_id: &str) -> syn::Result<Attr> {
    let mut kind = Attr::Default;

    for attr in attrs {
        if !attr.path().is_ident(derive_id) {
            continue;
        }

        let mut seen_skip = false;
        attr.parse_nested_meta(|meta| {
            if seen_skip {
                return Ok(());
            }

            if meta.path.is_ident("skip") {
                kind = Attr::Skip;
                seen_skip = true;
            } else if meta.path.is_ident("clone") {
                kind = Attr::Clone;
            } else {
                // 未知のオプション
                return Err(meta.error(consts::UNKNOWN_ATTR_OPT_MSG));
            }
            Ok(())
        })?;
        if seen_skip {
            break;
        }
    }

    Ok(kind)
}
