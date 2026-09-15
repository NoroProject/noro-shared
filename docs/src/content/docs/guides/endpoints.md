---
title: Your own endpoints
description: Declaring routes, who is allowed in, and what a handler receives.
---

A module can answer HTTP under its own prefix. Your mini-app talks to these; so can
anything else you point at them.

```rust
#[route(GET, "/me")]
fn me(req: HttpRequest) -> Result<Points> {
    let id = req.require_user()?;
    Ok(Points {
        points: store::user(id).get_or::<i64>("points")?,
    })
}
```

That answers `GET /api/modules/<your-id>/me`. The return type only has to serialize;
the master does the rest.

## Who is allowed in

```rust
#[route(POST, "/buy", auth = permission("noro.module.shop.buy"))]
fn buy(req: HttpRequest) -> Result<Receipt> { … }
```

| `auth` | Who gets through |
|---|---|
| `user` (default) | any signed-in player |
| `public` | anyone, including unauthenticated — for receiving webhooks |
| `permission("node")` | a player holding that permission |
| `admin("node")` | an admin token, or staff holding that permission |

The default is `user` on purpose: a public endpoint should be something you asked for,
never the result of a forgotten argument.

**Do not re-check permissions in the handler.** The master compared them against what
you declared before calling you — a request that fails never arrives. Checking again
just means two places to keep in step.

Permission nodes you use must be declared in the manifest and must start with
`noro.module.<id>.`, which is what keeps your nodes out of everybody else's branch and
gets them into the role editor's suggestions.

## What arrives

```rust
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub query: Value,           // the query string as one object
    pub body: Option<Value>,
    pub user: Option<Uuid>,     // empty only on a public endpoint
}
```

Helpers: `req.json::<T>()` for the body, `req.param("name")` for a query parameter,
`req.require_user()` when the endpoint needs a signed-in caller.

### Declarative Extractors

Handlers can declare their inputs directly as typed extractors in the signature:

```rust
#[route(POST, "/claim")]
fn claim(AuthUser(user): AuthUser, Json(body): Json<ClaimReq>) -> Result<Receipt> {
    // `user` is guaranteed Uuid; `body` is parsed `ClaimReq`
    Ok(Receipt { id: user })
}
```

Available extractors (in `noro_sdk::prelude::*`):
- `AuthUser(pub Uuid)` — extracts signed-in user id, or fails with an error if unauthenticated
- `OptionalUser(pub Option<Uuid>)` — extracts user id if available
- `Json<T>(pub T)` — deserializes JSON request body into `T`
- `QueryParams<T>(pub T)` — deserializes query string into `T`
- `RawParams(pub Value)` — provides raw query map
- `HttpRequest` — receives the complete underlying request object

## Custom Status Codes and Headers (`HttpResponse`)

By default, returning a serializable type from a route handler sends `200 OK` with `application/json`.
When you need custom HTTP status codes, redirects, or headers, return `HttpResponse`:

```rust
#[route(POST, "/item")]
fn create_item() -> Result<HttpResponse> {
    Ok(HttpResponse::new(201, json!({ "created": true }))
        .with_header("X-Custom-Header", "value"))
}

#[route(GET, "/legacy")]
fn legacy() -> Result<HttpResponse> {
    Ok(HttpResponse::redirect("/api/modules/shop/v2/items"))
}
```

The master parses the HTTP itself — there are no headers or cookies to handle here, and
the access decision was already made.

## Errors

Return `Err(ModuleError)` and the master turns it into a proper response:

```rust
return Err(ModuleError::not_found("no such lot"));
```

| Constructor | Means |
|---|---|
| `ModuleError::not_found(…)` | the object is not there |
| `ModuleError::invalid(…)` | the arguments do not pass |
| `ModuleError::conflict(…)` | the state does not allow it — insufficient funds, a duplicate |

Panicking is caught and becomes a module failure: the master does not fall over, but
your module gets a strike against it. Ten failures in five minutes and it is disabled
automatically.

## One path, one handler

Declaring the same method and path twice is refused at install time. "Which one runs" is
a question with no good answer, so it is not allowed to arise.
