//! Derive macro for declarative module settings.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let fields = match &ast.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(named) => &named.named,
            _ => panic!("Settings derive only works on structs with named fields"),
        },
        _ => panic!("Settings derive only works on structs"),
    };

    let mut reg_stmts = Vec::new();
    let mut load_fields = Vec::new();

    for field in fields {
        let f_ident = field.ident.as_ref().unwrap();
        let f_str = f_ident.to_string();
        let f_ty = &field.ty;

        let mut label = f_str.clone();
        for attr in &field.attrs {
            if attr.path().is_ident("setting") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("label") {
                        let value = meta.value()?;
                        let s: syn::LitStr = value.parse()?;
                        label = s.value();
                        Ok(())
                    } else {
                        Ok(())
                    }
                });
            }
        }

        let ty_str = quote!(#f_ty).to_string();
        let kind = if ty_str == "bool" {
            quote!(::noro_sdk::abi::manifest::SettingKind::Toggle)
        } else if ty_str.contains("int")
            || ty_str.contains("i64")
            || ty_str.contains("u64")
            || ty_str.contains("i32")
            || ty_str.contains("u32")
            || ty_str.contains("f64")
        {
            quote!(::noro_sdk::abi::manifest::SettingKind::Number)
        } else {
            quote!(::noro_sdk::abi::manifest::SettingKind::Text)
        };

        reg_stmts.push(quote! {
            reg.setting(#f_str, #kind, #label);
        });

        load_fields.push(quote! {
            #f_ident: ::noro_sdk::store::instance()
                .get(#f_str)?
                .unwrap_or(defaults.#f_ident)
        });
    }

    let expanded = quote! {
        impl #name {
            pub fn register(reg: &mut ::noro_sdk::Registration) {
                #(#reg_stmts)*
            }

            pub fn load() -> ::noro_sdk::Result<Self> {
                let defaults = Self::default();
                Ok(Self {
                    #(#load_fields),*
                })
            }
        }
    };

    expanded.into()
}
