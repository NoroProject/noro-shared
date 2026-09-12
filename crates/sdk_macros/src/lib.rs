//! Макросы для модулей Noro.
//!
//! `#[noro::module]` вешается на `impl`-блок и делает две вещи: превращает
//! помеченные методы в экспорты wasm и собирает из них декларацию, которую
//! мастер забирает вызовом `noro_register`.
//!
//! # Почему макрос на `impl`, а не реестр на каждом обработчике
//!
//! Обычный приём для «саморегистрации» — `inventory`/`linkme`: каждый атрибут
//! кладёт запись в секцию, а рантайм её обходит. Под
//! `wasm32-unknown-unknown` это работает ненадёжно: линкер выбрасывает секции,
//! на которые никто не ссылается. Макрос на `impl` видит все методы разом и
//! собирает список сам, не полагаясь на поведение линкера.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, FnArg, ImplItem, ItemImpl, LitStr, Pat, Token, Type};

mod args;

use args::{EventArgs, RouteArgs, TaskArgs};

/// Разметка модуля: экспорты и декларация для мастера.
///
/// # Что можно повесить на методы
///
/// Автодополнения внутри атрибутов не бывает — среда не знает грамматику
/// чужого макроса, пока он не раскрыт. Так же ведут себя `#[serde(…)]`,
/// `#[clap(…)]` и остальные. Поэтому весь допустимый набор перечислен здесь:
/// наведение на `#[noro::module]` показывает эту справку, а опечатка внутри
/// атрибута даёт ошибку со списком принимаемых значений.
///
/// | Атрибут | Сигнатура метода | Что делает |
/// |---|---|---|
/// | `#[event]` | `fn(E) -> Result<()>` | подписка; имя события берётся из типа `E` |
/// | `#[event(priority = high)]` | то же | `lowest` · `low` · `normal` · `high` · `highest` · `monitor` |
/// | `#[route(GET, "/путь")]` | `fn(HttpRequest) -> Result<T>` | ручка под `/api/modules/<id>/путь` |
/// | `#[route(POST, "/путь", auth = public)]` | то же | `public` · `user` · `permission("узел")` · `admin("узел")` |
/// | `#[task("1h")]` | `fn() -> Result<()>` | по расписанию: `30s` · `5m` · `1h` · `2d` |
/// | `#[register]` | `fn(&mut Registration)` | дополняет декларацию: поля настроек |
/// | `#[init]` | `fn() -> Result<()>` | разовая подготовка при включении |
///
/// Умолчания: приоритет — `normal`, доступ к ручке — `user`. Публичная ручка
/// объявляется намеренно, а не получается из забытого аргумента.
///
/// # Пример
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
    // Ручное дополнение декларации — вызывается внутри `noro_register`.
    let mut register_hook = None;
    // Инициализация при включении — отдельный экспорт.
    let mut init_hook = None;

    for item in &mut block.items {
        let ImplItem::Fn(method) = item else { continue };
        let name = method.sig.ident.clone();

        // Наши атрибуты снимаются: дальше компилятор их не знает.
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

        // Точка входа: автор дописывает в декларацию то, что не ложится на
        // атрибуты, — прежде всего поля настроек.
        if attr.path().is_ident("register") {
            register_hook = Some(name.clone());
            continue;
        }
        // Инициализация при включении. В отличие от `noro_register`, здесь у
        // модуля уже есть выданные возможности, и он может ходить в хранилище.
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
                None => return err(&name, "обработчику события нужен аргумент — само событие"),
            };
            let priority = args.priority_tokens();

            // Имя события берётся из типа через `Event::NAME`: подписаться на
            // одно, а принять структуру другого теперь невозможно.
            events.push(quote! {
                reg.events.push(::noro_sdk::abi::registration::EventReg {
                    name: <#arg_ty as ::noro_sdk::abi::events::Event>::NAME.to_string(),
                    handler: #handler.to_string(),
                    priority: #priority,
                });
            });

            exports.push(quote! {
                #[::noro_sdk::extism_pdk::plugin_fn]
                pub fn #name(
                    ::noro_sdk::extism_pdk::Json(event): ::noro_sdk::extism_pdk::Json<#arg_ty>,
                ) -> ::noro_sdk::extism_pdk::FnResult<()> {
                    <#self_ty>::#name(event)?;
                    Ok(())
                }
            });
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

            // Ответ уходит как `serde_json::Value`: тип возврата обработчика
            // знать незачем, достаточно того, что он сериализуется.
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
            /// Инициализация модуля. Мастер зовёт её после включения.
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

        /// Что модуль умеет. Мастер зовёт это при установке и при включении.
        ///
        /// Возможностей на этот момент модулю не выдано: декларацию он собирает
        /// сам из себя, ничего не спрашивая у хоста.
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
            // Ручное дополнение идёт последним: оно может опереться на то,
            // что уже собрано из атрибутов.
            #register_call
            Ok(::noro_sdk::extism_pdk::Json(reg))
        }
    }
    .into()
}

/// `#[event]` без скобок — обычный приоритет.
fn parse_event(attr: &syn::Attribute) -> syn::Result<EventArgs> {
    match &attr.meta {
        syn::Meta::Path(_) => Ok(EventArgs::default()),
        _ => attr.parse_args::<EventArgs>(),
    }
}

/// Тип первого аргумента метода — он же тип события.
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

/// Разбор строки — для `#[task("1h")]` в краткой форме.
impl Parse for TaskArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            return Ok(TaskArgs {
                every: input.parse()?,
            });
        }
        let key: syn::Ident = input.parse()?;
        if key != "every" {
            return Err(syn::Error::new(key.span(), "ожидается every = \"1h\""));
        }
        input.parse::<Token![=]>()?;
        Ok(TaskArgs {
            every: input.parse()?,
        })
    }
}
