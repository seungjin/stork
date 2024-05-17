mod bindings;

use crate::bindings::seungjin::stork::request::get;
use crate::bindings::seungjin::stork::s3::foo;

use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_spin_app(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let b = get("https://catfact.ninja/fact").unwrap();
    let c = foo();

    println!("{b} {c}");

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build())
}
