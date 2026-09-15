//! Generator for declarative inter-module RPC handlers (`#[rpc]`).

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ImplItemFn, LitStr, Type};

pub fn process_rpc(
    self_ty: &Type,
    method: &ImplItemFn,
    attr: &syn::Attribute,
) -> syn::Result<(TokenStream, TokenStream)> {
    let name = method.sig.ident.clone();
    let rpc_name: LitStr = match attr.parse_args::<LitStr>() {
        Ok(a) => a,
        Err(_) => LitStr::new(&name.to_string(), name.span()),
    };

    let export_name = format_ident!("rpc_{}", name);
    let handler_str = export_name.to_string();

    let call_args: Vec<_> = method
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(t) => {
                let ty = &t.ty;
                Some(quote!(<#ty as ::noro_sdk::extract::FromRequest>::from_request(&request)?))
            }
            _ => None,
        })
        .collect();

    let registration = quote! {
        reg.routes.push(::noro_sdk::abi::registration::RouteReg {
            method: "RPC".to_string(),
            path: #rpc_name.to_string(),
            handler: #handler_str,
            auth: ::noro_sdk::abi::manifest::Auth::User,
        });
    };

    let export = quote! {
        #[::noro_sdk::extism_pdk::plugin_fn]
        pub fn #export_name(
            ::noro_sdk::extism_pdk::Json(payload): ::noro_sdk::extism_pdk::Json<::serde_json::Value>,
        ) -> ::noro_sdk::extism_pdk::FnResult<::noro_sdk::extism_pdk::Json<::serde_json::Value>> {
            let request = ::noro_sdk::abi::http::HttpRequest {
                method: "RPC".to_string(),
                path: #rpc_name.to_string(),
                query: ::serde_json::json!({}),
                body: Some(payload),
                user: None,
            };
            let res = <#self_ty>::#name(#(#call_args),*)?;
            let val = ::serde_json::to_value(res)?;
            Ok(::noro_sdk::extism_pdk::Json(val))
        }
    };

    Ok((registration, export))
}
