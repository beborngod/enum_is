#![doc = include_str!("../README.md")]

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use std::collections::BTreeMap;
use syn::{Attribute, Data, DeriveInput, Error, Fields, parse_macro_input};
use syn::spanned::Spanned;

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

            let attrs = match parse_enum_is_attrs(&variant.attrs) {
                Ok(attrs) => attrs,
                Err(err) => return Some(err.to_compile_error()),
            };

            if attrs.ignore {
                return None;
            }

            let method_name_str = attrs
                .rename
                .unwrap_or_else(|| format!("is_{}", variant_ident.to_string().to_snake_case()));

            let method_ident = syn::Ident::new(&method_name_str, variant_ident.span());

            let pat: TokenStream2 = match &variant.fields {
                Fields::Unit => quote! { Self::#variant_ident },
                Fields::Unnamed(_) => quote! { Self::#variant_ident (..) },
                Fields::Named(_) => quote! { Self::#variant_ident { .. } },
            };

            for group in attrs.groups {
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

struct EnumIsAttrs {
    ignore: bool,
    rename: Option<String>,
    groups: Vec<String>,
}

fn parse_enum_is_attrs(attrs: &[Attribute]) -> Result<EnumIsAttrs, Error> {
    let mut parsed = EnumIsAttrs {
        ignore: false,
        rename: None,
        groups: Vec::new(),
    };
    let mut enum_is_span: Option<Span> = None;

    for attr in attrs {
        if !attr.path().is_ident("enum_is") {
            continue;
        }

        if enum_is_span.is_none() {
            enum_is_span = Some(attr.span());
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("ignore") {
                parsed.ignore = true;
                return Ok(());
            }

            if meta.path.is_ident("rename") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                parsed.rename = Some(lit.value());
                return Ok(());
            }

            if meta.path.is_ident("group") {
                let lit: syn::LitStr = meta.value()?.parse()?;
                parsed.groups.push(lit.value());
                return Ok(());
            }

            Err(meta.error("unsupported enum_is attribute"))
        })?;
    }

    if parsed.ignore && (parsed.rename.is_some() || !parsed.groups.is_empty()) {
        let span = enum_is_span.unwrap_or_else(Span::call_site);
        return Err(Error::new(
            span,
            "#[enum_is(ignore)] cannot be combined with rename/group",
        ));
    }

    Ok(parsed)
}
