use proc_macro::TokenStream;

use quote::quote;
use syn::{DataStruct, DeriveInput};

fn extract_struct_data(ast: &DeriveInput) -> &DataStruct {
	if let syn::Data::Struct(data) = &ast.data {
		data
	} else {
		panic!("Can only be derived for structs");
	}
}

pub fn implement(ast: DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let _ = extract_struct_data(&ast);

	TokenStream::from(quote!(
		impl NewId for #name {
			const TABLE: &'static str = #name::TABLE;

			fn id(&self) -> String {
				self.id.clone()
			}

			fn record(&self) -> String {
				self.record.clone()
			}

			fn new(&self) -> Self {
				Self((Sefl::TABLE), self.random())
			}

			fn random() -> Self {
				Self {
					id: RecordId::random(Self::TABLE),
					record: Self::TABLE.to_string(),
				}
			}
		}
	))
}
