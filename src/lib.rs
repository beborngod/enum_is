use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(EnumIs, attributes(enum_is))]
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

    let methods = data_enum.variants.into_iter().filter_map(|variant| {
        let variant_ident = &variant.ident;

        // #[enum_is(ignore)]
        if variant_is_ignored(&variant.attrs) {
            return None;
        }

        let method_name_str = format!("is_{}", variant_ident.to_string().to_snake_case());
        let method_ident = syn::Ident::new(&method_name_str, variant_ident.span());

        let pat = match &variant.fields {
            Fields::Unit => quote! { Self::#variant_ident },
            Fields::Unnamed(_) => quote! { Self::#variant_ident (..) },
            Fields::Named(_) => quote! { Self::#variant_ident { .. } },
        };

        Some(quote! {
            #[inline]
            pub fn #method_ident(&self) -> bool {
                matches!(self, #pat)
            }
        })
    });

    let expanded = quote! {
        impl #enum_name {
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}

/// #[enum_is(ignore)]
fn variant_is_ignored(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if !attr.path().is_ident("enum_is") {
            continue;
        }

        let mut ignore = false;

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("ignore") {
                ignore = true;
            }
            Ok(())
        });

        if ignore {
            return true;
        }
    }

    false
}
