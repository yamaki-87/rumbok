use proc_macro::TokenStream;
mod setter;

#[proc_macro_derive(Setter)]
pub fn derive_setter(input: TokenStream) -> TokenStream {
    setter::setter(input.into()).into()
}
