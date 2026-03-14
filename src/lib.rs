use worker::*;
use serde_json;

#[event(fetch)]
async fn main(
    _req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    let router = Router::new();

    router
    .get_async("/extract", |_, _| async move {
        Response::ok("Extracting!")
    })
    .or_else_any_method_async("/:catchall", |_, _| async move {
        let body = serde_json::json!({
            "error": "Not Found",
            "status": 404
        });
        Response::from_json(&body)
    })
    .run(_req, _env).await
}