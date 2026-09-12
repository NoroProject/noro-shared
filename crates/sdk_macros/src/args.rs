//! Parsing the arguments of the handler attributes.

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Token};

/// `#[event]` · `#[event(priority = high)]`
#[derive(Default)]
pub struct EventArgs {
    pub priority: Option<Ident>,
}

impl EventArgs {
    /// The priority as an enum value. Normal by default.
    pub fn priority_tokens(&self) -> TokenStream {
        match &self.priority {
            None => quote!(::noro_sdk::abi::manifest::Priority::Normal),
            Some(p) => {
                let variant = match p.to_string().as_str() {
                    "lowest" => quote!(Lowest),
                    "low" => quote!(Low),
                    "normal" => quote!(Normal),
                    "high" => quote!(High),
                    "highest" => quote!(Highest),
                    "monitor" => quote!(Monitor),
                    other => {
                        let message = format!(
                            "unknown priority `{other}`: lowest, low, normal, high, highest, monitor"
                        );
                        return syn::Error::new(p.span(), message).to_compile_error();
                    }
                };
                quote!(::noro_sdk::abi::manifest::Priority::#variant)
            }
        }
    }
}

impl Parse for EventArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self::default());
        }
        let key: Ident = input.parse()?;
        if key != "priority" {
            return Err(syn::Error::new(
                key.span(),
                "an event only takes priority = …",
            ));
        }
        input.parse::<Token![=]>()?;
        Ok(Self {
            priority: Some(input.parse()?),
        })
    }
}

/// `#[route(GET, "/me")]` · `#[route(POST, "/buy", auth = permission("node"))]`
pub struct RouteArgs {
    pub method: LitStr,
    pub path: LitStr,
    pub auth: Option<AuthArg>,
}

pub enum AuthArg {
    Public,
    User,
    Permission(LitStr),
    Admin(LitStr),
}

impl RouteArgs {
    /// Who is let in. Any signed-in user by default: a public endpoint has to
    /// be declared deliberately, not end up public through a forgotten
    /// argument.
    pub fn auth_tokens(&self) -> TokenStream {
        match &self.auth {
            None | Some(AuthArg::User) => quote!(::noro_sdk::abi::manifest::Auth::User),
            Some(AuthArg::Public) => quote!(::noro_sdk::abi::manifest::Auth::Public),
            Some(AuthArg::Permission(node)) => {
                quote!(::noro_sdk::abi::manifest::Auth::Permission(#node.to_string()))
            }
            Some(AuthArg::Admin(node)) => {
                quote!(::noro_sdk::abi::manifest::Auth::Admin(#node.to_string()))
            }
        }
    }
}

impl Parse for RouteArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // The method is written bare — `GET`, not `"GET"`: that way the
        // request line reads at a glance, and a typo is caught by the check at
        // install time.
        let method: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let path: LitStr = input.parse()?;

        let mut auth = None;
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let key: Ident = input.parse()?;
            if key != "auth" {
                return Err(syn::Error::new(
                    key.span(),
                    "an endpoint only takes auth = …",
                ));
            }
            input.parse::<Token![=]>()?;
            auth = Some(input.parse()?);
        }

        Ok(Self {
            method: LitStr::new(&method.to_string().to_uppercase(), method.span()),
            path,
            auth,
        })
    }
}

impl Parse for AuthArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kind: Ident = input.parse()?;
        match kind.to_string().as_str() {
            "public" => Ok(AuthArg::Public),
            "user" => Ok(AuthArg::User),
            "permission" | "admin" => {
                let inner;
                syn::parenthesized!(inner in input);
                let node: LitStr = inner.parse()?;
                if kind == "permission" {
                    Ok(AuthArg::Permission(node))
                } else {
                    Ok(AuthArg::Admin(node))
                }
            }
            other => Err(syn::Error::new(
                kind.span(),
                format!(
                    "unknown access `{other}`: public, user, permission(\"node\"), admin(\"node\")"
                ),
            )),
        }
    }
}

/// `#[task("1h")]` · `#[task(every = "1h")]`
pub struct TaskArgs {
    pub every: LitStr,
}
