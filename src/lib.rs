use proc_macro::TokenStream;
mod consts;
mod getter;
mod setter;
mod utils;
#[proc_macro_derive(Setter)]
pub fn derive_setter(input: TokenStream) -> TokenStream {
    setter::setter(input.into()).into()
}

#[proc_macro_derive(Getter)]
pub fn derive_getter(input: TokenStream) -> TokenStream {
    getter::getter(input.into()).into()
}

#[proc_macro_derive(Data)]
pub fn derive_data(input: TokenStream) -> TokenStream {
    TokenStream::from_iter(vec![derive_setter(input.clone()), derive_getter(input)])
}
