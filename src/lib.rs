#![doc = include_str!("../README.md")]

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::collections::BTreeMap;
use syn::{Attribute, Data, DeriveInput, Error, Fields, parse_macro_input};

#[proc_macro_derive(EnumIs, attributes(enum_is))]
pub fn derive_enum_is(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = input.ident;

    let data_enum = match input.data {
        Data::Enum(e) => e,
        _ => {
            return Error::new_spanned(enum_name, "#[derive(EnumIs)] can only be used on enums")
                .to_compile_error()
                .into();
        }
    };

    let mut group_map: BTreeMap<String, Vec<TokenStream2>> = BTreeMap::new();

    let methods: Vec<TokenStream2> = data_enum
        .variants
        .into_iter()
        .filter_map(|variant| {
            let variant_ident = &variant.ident;

            if variant_is_ignored(&variant.attrs) {
                return None;
            }

            let method_name_str = variant_rename(&variant.attrs)
                .unwrap_or_else(|| format!("is_{}", variant_ident.to_string().to_snake_case()));

            let method_ident = syn::Ident::new(&method_name_str, variant_ident.span());

            let pat: TokenStream2 = match &variant.fields {
                Fields::Unit => quote! { Self::#variant_ident },
                Fields::Unnamed(_) => quote! { Self::#variant_ident (..) },
                Fields::Named(_) => quote! { Self::#variant_ident { .. } },
            };

            for group in variant_groups(&variant.attrs) {
                group_map.entry(group).or_default().push(pat.clone());
            }

            Some(quote! {
                #[inline]
                pub fn #method_ident(&self) -> bool {
                    matches!(self, #pat)
                }
            })
        })
        .collect();

    let group_methods: Vec<TokenStream2> = group_map
        .into_iter()
        .map(|(name, patterns)| {
            let method_ident = syn::Ident::new(&name, Span::call_site());
            quote! {
                #[inline]
                pub fn #method_ident(&self) -> bool {
                    matches!(self, #( #patterns )|* )
                }
            }
        })
        .collect();

    let expanded = quote! {
        impl #enum_name {
            #(#methods)*
            #(#group_methods)*
        }
    };

    TokenStream::from(expanded)
}

/// `#[enum_is(ignore)]`
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

/// `#[enum_is(rename = "...")]`
fn variant_rename(attrs: &[Attribute]) -> Option<String> {
    let mut result: Option<String> = None;

    for attr in attrs {
        if !attr.path().is_ident("enum_is") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                result = Some(lit.value());
            }
            Ok(())
        });
    }

    result
}

/// `#[enum_is(group = "...")]`
fn variant_groups(attrs: &[Attribute]) -> Vec<String> {
    let mut result = Vec::new();

    for attr in attrs {
        if !attr.path().is_ident("enum_is") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("group") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                result.push(lit.value());
            }
            Ok(())
        });
    }

    result
}
