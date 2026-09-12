//! Macros for Noro modules.
//!
//! `#[noro::module]` goes on an `impl` block and does two things: it turns the
//! annotated methods into wasm exports, and it assembles them into the
//! declaration the master collects by calling `noro_register`.
//!
//! # Why a macro on the `impl` instead of a registry per handler
//!
//! The usual trick for self-registration is `inventory`/`linkme`: every
//! attribute drops a record into a section, and the runtime walks it. On
//! `wasm32-unknown-unknown` that is unreliable: the linker discards sections
//! nothing refers to. A macro on the `impl` sees every method at once and
//! builds the list itself, relying on no linker behaviour.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, FnArg, ImplItem, ItemImpl, LitStr, Pat, Token, Type};

mod args;

use args::{EventArgs, RouteArgs, TaskArgs};

/// The module's markup: exports plus the declaration for the master.
///
/// # What you can put on methods
///
/// There is no autocomplete inside attributes — the editor does not know
/// another macro's grammar until it is expanded. `#[serde(…)]`, `#[clap(…)]`
/// and the rest behave the same way. So the entire accepted set is listed
/// here: hovering `#[noro::module]` shows this reference, and a typo inside an
/// attribute produces an error listing the accepted values.
///
/// | Attribute | Method signature | What it does |
/// |---|---|---|
/// | `#[event]` | `fn(E) -> Result<()>` | a subscription; the event name comes from the type `E` |
/// | `#[event(priority = high)]` | same | `lowest` · `low` · `normal` · `high` · `highest` · `monitor` |
/// | `#[event("mod.shop.purchase")]` | `fn(T) -> Result<()>` | another module's event, which has no type to name it |
/// | `#[route(GET, "/path")]` | `fn(HttpRequest) -> Result<T>` | an endpoint under `/api/modules/<id>/path` |
/// | `#[route(POST, "/path", auth = public)]` | same | `public` · `user` · `permission("node")` · `admin("node")` |
/// | `#[task("1h")]` | `fn() -> Result<()>` | on a schedule: `30s` · `5m` · `1h` · `2d` |
/// | `#[register]` | `fn(&mut Registration)` | extends the declaration: settings fields |
/// | `#[init]` | `fn() -> Result<()>` | one-off preparation when enabled |
///
/// Defaults: priority is `normal`, endpoint access is `user`. A public
/// endpoint is declared deliberately, never the result of a forgotten
/// argument.
///
/// # Example
///
/// ```ignore
/// #[noro::module]
/// impl Shop {
///     #[register]
///     fn setup(reg: &mut Registration) {
///         reg.setting("tax", SettingKind::Number, "mod-shop-tax").default(5);
///     }
///
///     #[event(priority = high)]
///     fn on_transfer(e: BankPreTransfer) -> Result<()> { Ok(()) }
///
///     #[route(POST, "/buy", auth = permission("noro.module.shop.buy"))]
///     fn buy(req: HttpRequest) -> Result<u64> { Ok(0) }
/// }
/// ```
#[proc_macro_attribute]
pub fn module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut block = parse_macro_input!(item as ItemImpl);
    let self_ty = block.self_ty.clone();

    let mut exports = Vec::new();
    let mut events = Vec::new();
    let mut routes = Vec::new();
    let mut tasks = Vec::new();
    // Manual additions to the declaration — called inside `noro_register`.
    let mut register_hook = None;
    // Initialization on enable — a separate export.
    let mut init_hook = None;

    for item in &mut block.items {
        let ImplItem::Fn(method) = item else { continue };
        let name = method.sig.ident.clone();

        // Our attributes are stripped: past this point the compiler does not know them.
        let mut kind = None;
        method.attrs.retain(|attr| {
            let path = attr.path();
            if path.is_ident("event")
                || path.is_ident("route")
                || path.is_ident("task")
                || path.is_ident("register")
                || path.is_ident("init")
            {
                kind = Some(attr.clone());
                false
            } else {
                true
            }
        });
        let Some(attr) = kind else { continue };

        // The entry point: the author adds to the declaration whatever does
        // not fit an attribute — settings fields above all.
        if attr.path().is_ident("register") {
            register_hook = Some(name.clone());
            continue;
        }
        // Initialization on enable. Unlike `noro_register`, by this point the
        // module has its granted capabilities and can reach the store.
        if attr.path().is_ident("init") {
            init_hook = Some(name.clone());
            continue;
        }

        let handler = name.to_string();

        if attr.path().is_ident("event") {
            let args = match parse_event(&attr) {
                Ok(a) => a,
                Err(e) => return e.to_compile_error().into(),
            };
            let arg_ty = match first_arg_type(method) {
                Some(t) => t,
                None => {
                    return err(
                        &name,
                        "an event handler needs an argument — the event itself",
                    )
                }
            };
            let priority = args.priority_tokens();

            // `&mut E` означает отменяемое событие: обработчик правит его на
            // месте, и правки должны уехать обратно мастеру. Различается это по
            // самому типу, а не отдельным атрибутом: подпись уже всё говорит, а
            // атрибут можно поставить не тот.
            let mutable = matches!(&arg_ty, Type::Reference(r) if r.mutability.is_some());
            let inner = match &arg_ty {
                Type::Reference(r) => (*r.elem).clone(),
                other => other.clone(),
            };

            // The name comes from the type via `Event::NAME`: subscribing to
            // one event and accepting another's struct is impossible that way.
            // A written name is accepted only for another module's event, which
            // has no such type — see `EventArgs::name`.
            // Не `name`: так уже назван идентификатор метода, и затенение
            // подставило бы строку туда, где макрос объявляет функцию.
            let event_name = match &args.name {
                Some(lit) => quote!(#lit.to_string()),
                None => {
                    quote!(<#inner as ::noro_sdk::abi::events::Event>::NAME.to_string())
                }
            };
            events.push(quote! {
                reg.events.push(::noro_sdk::abi::registration::EventReg {
                    name: #event_name,
                    handler: #handler.to_string(),
                    priority: #priority,
                });
            });

            // Отменяемое возвращает событие обратно, уведомительное — ничего:
            // мастеру нечего делать с ответом на то, что уже случилось.
            let export = if mutable {
                quote! {
                    #[::noro_sdk::extism_pdk::plugin_fn]
                    pub fn #name(
                        ::noro_sdk::extism_pdk::Json(mut event): ::noro_sdk::extism_pdk::Json<#inner>,
                    ) -> ::noro_sdk::extism_pdk::FnResult<
                        ::noro_sdk::extism_pdk::Json<#inner>,
                    > {
                        <#self_ty>::#name(&mut event)?;
                        Ok(::noro_sdk::extism_pdk::Json(event))
                    }
                }
            } else {
                quote! {
                    #[::noro_sdk::extism_pdk::plugin_fn]
                    pub fn #name(
                        ::noro_sdk::extism_pdk::Json(event): ::noro_sdk::extism_pdk::Json<#inner>,
                    ) -> ::noro_sdk::extism_pdk::FnResult<()> {
                        <#self_ty>::#name(event)?;
                        Ok(())
                    }
                }
            };
            exports.push(export);
        } else if attr.path().is_ident("route") {
            let args = match attr.parse_args::<RouteArgs>() {
                Ok(a) => a,
                Err(e) => return e.to_compile_error().into(),
            };
            let (method_lit, path_lit) = (&args.method, &args.path);
            let auth = args.auth_tokens();

            routes.push(quote! {
                reg.routes.push(::noro_sdk::abi::registration::RouteReg {
                    method: #method_lit.to_string(),
                    path: #path_lit.to_string(),
                    handler: #handler.to_string(),
                    auth: #auth,
                });
            });

            // The answer leaves as a `serde_json::Value`: there is no need to
            // know the handler's return type, only that it serializes.
            exports.push(quote! {
                #[::noro_sdk::extism_pdk::plugin_fn]
                pub fn #name(
                    ::noro_sdk::extism_pdk::Json(request): ::noro_sdk::extism_pdk::Json<::noro_sdk::abi::HttpRequest>,
                ) -> ::noro_sdk::extism_pdk::FnResult<::noro_sdk::extism_pdk::Json<::noro_sdk::serde_json::Value>> {
                    let answer = <#self_ty>::#name(request)?;
                    Ok(::noro_sdk::extism_pdk::Json(::noro_sdk::serde_json::to_value(answer)?))
                }
            });
        } else {
            let args = match attr.parse_args::<TaskArgs>() {
                Ok(a) => a,
                Err(e) => return e.to_compile_error().into(),
            };
            let every = &args.every;

            tasks.push(quote! {
                reg.tasks.push(::noro_sdk::abi::registration::TaskReg {
                    handler: #handler.to_string(),
                    every: #every.to_string(),
                });
            });

            exports.push(quote! {
                #[::noro_sdk::extism_pdk::plugin_fn]
                pub fn #name(_: ()) -> ::noro_sdk::extism_pdk::FnResult<()> {
                    <#self_ty>::#name()?;
                    Ok(())
                }
            });
        }
    }

    let register = format_ident!("noro_register");
    let register_call = match &register_hook {
        Some(name) => quote!(<#self_ty>::#name(&mut reg);),
        None => quote!(),
    };
    let init_export = match &init_hook {
        Some(name) => quote! {
            /// Module initialization. The master calls it after enabling.
            #[::noro_sdk::extism_pdk::plugin_fn]
            pub fn noro_init(_: ()) -> ::noro_sdk::extism_pdk::FnResult<()> {
                <#self_ty>::#name()?;
                Ok(())
            }
        },
        None => quote!(),
    };

    quote! {
        #block

        #(#exports)*

        #init_export

        /// What the module can do. The master calls this at install time and
        /// on enable.
        ///
        /// The module has no capabilities granted at that point: it assembles
        /// the declaration out of itself, asking the host for nothing.
        #[::noro_sdk::extism_pdk::plugin_fn]
        pub fn #register(
            _: (),
        ) -> ::noro_sdk::extism_pdk::FnResult<
            ::noro_sdk::extism_pdk::Json<::noro_sdk::abi::registration::Registration>,
        > {
            let mut reg = ::noro_sdk::abi::registration::Registration::default();
            #(#events)*
            #(#routes)*
            #(#tasks)*
            // The manual addition goes last: it can build on what the
            // attributes already collected.
            #register_call
            Ok(::noro_sdk::extism_pdk::Json(reg))
        }
    }
    .into()
}

/// `#[event]` without parentheses means the normal priority.
fn parse_event(attr: &syn::Attribute) -> syn::Result<EventArgs> {
    match &attr.meta {
        syn::Meta::Path(_) => Ok(EventArgs::default()),
        _ => attr.parse_args::<EventArgs>(),
    }
}

/// The type of the method's first argument, which is the event type.
fn first_arg_type(method: &syn::ImplItemFn) -> Option<Type> {
    method.sig.inputs.iter().find_map(|arg| match arg {
        FnArg::Typed(t) => match &*t.pat {
            Pat::Ident(_) | Pat::Wild(_) | Pat::Struct(_) | Pat::TupleStruct(_) => {
                Some((*t.ty).clone())
            }
            _ => Some((*t.ty).clone()),
        },
        FnArg::Receiver(_) => None,
    })
}

fn err(at: &syn::Ident, message: &str) -> TokenStream {
    syn::Error::new(at.span(), message)
        .to_compile_error()
        .into()
}

/// String parsing — for the short form `#[task("1h")]`.
impl Parse for TaskArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            return Ok(TaskArgs {
                every: input.parse()?,
            });
        }
        let key: syn::Ident = input.parse()?;
        if key != "every" {
            return Err(syn::Error::new(key.span(), "expected every = \"1h\""));
        }
        input.parse::<Token![=]>()?;
        Ok(TaskArgs {
            every: input.parse()?,
        })
    }
}
