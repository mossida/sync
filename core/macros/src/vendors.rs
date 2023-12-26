use proc_macro::TokenStream;

use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{DataEnum, DeriveInput};

fn extract_enum_data(ast: &DeriveInput) -> &DataEnum {
    if let syn::Data::Enum(data) = &ast.data {
        data
    } else {
        panic!("Can only be derived for enums");
    }
}

pub fn implement(ast: DeriveInput) -> TokenStream {
    let data = extract_enum_data(&ast);

    let enum_name = &ast.ident;
    let variants = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let struct_name: proc_macro2::TokenStream =
            syn::parse_str(format!("vendors::{}::adapter::{}", variant_name.to_string().to_lowercase(), variant_name.to_string()).as_str()).unwrap();

        quote_spanned! {
                variant.span()=>
                #enum_name::#variant_name => {
                    const _: fn() = || {
                        fn impl_all<T: ?Sized + ractor::actor::Actor>() {}
                        impl_all::<#struct_name>();
                    };

                    let (_macro_ref, _) = ractor::actor::Actor::spawn_linked(None, #struct_name {}, configuration, supervisor).await?;
                    Ok(_macro_ref.get_cell())
                }
            }
    });

    // Convert the expanded code into a token stream and return it
    TokenStream::from(quote! {
        impl #enum_name {
            pub async fn build(&self, configuration: serde_json::Value, supervisor: ractor::ActorCell) -> utils::types::Result<ractor::ActorCell> {
                match self {
                    #(#variants)*
                }
            }
        }
    })
}
