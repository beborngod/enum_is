use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(EnumIs)]
pub fn derive_enum_is(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = input.ident;

    let data_enum = match input.data {
        Data::Enum(e) => e,
        _ => {
            return syn::Error::new_spanned(
                enum_name,
                "#[derive(EnumIs)] can only be used on enums",
            )
            .to_compile_error()
            .into();
        }
    };

    let methods = data_enum.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        let method_name_str = format!("is_{}", variant_ident.to_string().to_snake_case());
        let method_ident = syn::Ident::new(&method_name_str, variant_ident.span());

        let pat = match &variant.fields {
            Fields::Unit => quote! { Self::#variant_ident },
            Fields::Unnamed(_) => quote! { Self::#variant_ident (..) },
            Fields::Named(_) => quote! { Self::#variant_ident { .. } },
        };

        quote! {
            #[inline]
            pub fn #method_ident(&self) -> bool {
                matches!(self, #pat)
            }
        }
    });

    let expanded = quote! {
        impl #enum_name {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}
