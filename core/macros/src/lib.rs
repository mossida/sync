use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

mod id;
mod vendors;

#[proc_macro_derive(VendorBuilder, attributes(vendor))]
pub fn vendors_build(item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as DeriveInput);
	vendors::implement(input)
}

#[proc_macro_derive(Id, attributes(actor))]
pub fn custom_id(item: TokenStream) -> TokenStream {
	let input = parse_macro_input!(item as DeriveInput);
	id::implement(input)
}
