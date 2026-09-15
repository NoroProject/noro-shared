//! Generator for declarative WebSocket action handlers (`#[ws_action]`).

use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, Ident, ImplItemFn, LitStr, Type};

pub struct WsActionItem {
    pub action: LitStr,
    pub name: Ident,
    pub call_args: Vec<TokenStream>,
}

impl WsActionItem {
    pub fn from_fn(method: &ImplItemFn, action: LitStr) -> Self {
        let name = method.sig.ident.clone();
        let call_args: Vec<_> = method
            .sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                FnArg::Typed(t) => {
                    let ty = &t.ty;
                    Some(quote!(<#ty as ::noro_sdk::extract::FromWebMessage>::from_web_message(&msg)?))
                }
                _ => None,
            })
            .collect();

        Self {
            action,
            name,
            call_args,
        }
    }
}

pub fn generate_dispatcher(self_ty: &Type, items: &[WsActionItem]) -> (TokenStream, TokenStream) {
    if items.is_empty() {
        return (quote!(), quote!());
    }

    let registration = quote! {
        reg.events.push(::noro_sdk::abi::registration::EventReg {
            name: "web.message".to_string(),
            handler: "__noro_ws_dispatch".to_string(),
            priority: ::noro_sdk::abi::manifest::Priority::Normal,
        });
    };

    let arms: Vec<_> = items
        .iter()
        .map(|item| {
            let action = &item.action;
            let name = &item.name;
            let call_args = &item.call_args;
            quote! {
                #action => {
                    let _ = <#self_ty>::#name(#(#call_args),*)?;
                }
            }
        })
        .collect();

    let export = quote! {
        #[::noro_sdk::extism_pdk::plugin_fn]
        pub fn __noro_ws_dispatch(
            ::noro_sdk::extism_pdk::Json(msg): ::noro_sdk::extism_pdk::Json<::noro_sdk::abi::events::WebMessage>,
        ) -> ::noro_sdk::extism_pdk::FnResult<()> {
            let action_str = msg.payload.get("action")
                .or_else(|| msg.payload.get("type"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            match action_str {
                #(#arms)*
                _ => {}
            }
            Ok(())
        }
    };

    (registration, export)
}
